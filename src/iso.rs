use std::{iter::Peekable, str::Chars};

use crate::{calendar::IsoCalendar, CalendarProtocol};

#[derive(Debug)]
pub struct IsoDate {
    pub year: i32,
    pub month: u8,
    pub day: u16,
}

impl IsoDate {
    pub(crate) fn to_epoch_second(&self) -> i64 {
        let mut days: i64 = (self.year - 1970) as i64 * 365i64;
        days += (self.year - 1969).div_euclid(4) as i64;
        days -= (self.year - 1901).div_euclid(100) as i64;
        days += (self.year - 1601).div_euclid(400) as i64;
        days += IsoCalendar.day_of_year(self.year, self.month, self.day) as i64;
        days * 24 * 60 * 60
    }

    pub(crate) fn from_epoch_second(secs: i64) -> Self {
        const SEC_PER_DAY: i64 = 86400;
        const DAYS_PER_400Y: i64 = 365 * 400 + 97;
        const DAYS_PER_100Y: i64 = 365 * 100 + 24;
        const DAYS_PER_4Y: i64 = 365 * 4 + 1;
        // 2000-03-01 (mod 400 year, immediately after feb29
        const LEAPOCH: i64 = 946684800 + 86400 * (31 + 29);
        const DAYS_IN_MONTH: [i64; 12] = [31, 30, 31, 30, 31, 31, 30, 31, 30, 31, 31, 29];
        let mut days = (secs - LEAPOCH).div_euclid(SEC_PER_DAY);
        let qc_cycle = days.div_euclid(DAYS_PER_400Y);
        days = days.rem_euclid(DAYS_PER_400Y);
        let mut c_cycle = days.div_euclid(DAYS_PER_100Y);
        if c_cycle == 4 {
            c_cycle -= 1;
        }
        days -= c_cycle * DAYS_PER_100Y;
        let mut q_cycle = days.div_euclid(DAYS_PER_4Y);
        if q_cycle == 25 {
            q_cycle -= 1;
        }
        days -= q_cycle * DAYS_PER_4Y;
        let mut rem_years = days.div_euclid(365);
        if rem_years == 4 {
            rem_years -= 1;
        }
        days -= rem_years * 365;
        let years = rem_years + 4 * q_cycle + 100 * c_cycle + 400 * qc_cycle;
        let mut months = 3;
        for m in DAYS_IN_MONTH {
            if days < m {
                break;
            }
            months += 1;
            days -= m;
        }
        Self {
            year: (years + 2000) as i32,
            month: months as u8,
            day: days as u16,
        }
    }
}

#[derive(Debug, Default, Clone, Copy)]
pub struct IsoTime {
    pub hour: u8,
    pub minute: u8,
    pub second: u8,
    pub millisecond: u16,
    pub microsecond: u16,
    pub nanosecond: u16,
}

impl IsoTime {
    pub(crate) fn to_second(&self) -> i32 {
        self.hour as i32 * 60 * 60 + self.minute as i32 * 60 + self.second as i32
    }

    pub(crate) fn has_nanosecond(&self) -> bool {
        self.millisecond != 0 || self.microsecond != 0 || self.nanosecond != 0
    }

    pub(crate) fn to_nanosecond(&self) -> i64 {
        self.to_second() as i64 * 1000_000_000
            + self.millisecond as i64 * 1000_000
            + self.microsecond as i64 * 1000
            + self.nanosecond as i64
    }
}

#[derive(Debug, Clone, Copy)]
pub(crate) struct IsoNumericOffset {
    is_neg: bool,
    hour: u8,
    minute: u8,
}

impl IsoNumericOffset {
    pub(crate) fn to_seconds(&self) -> i32 {
        let h = self.hour as i32;
        let m = self.minute as i32;
        if self.is_neg {
            -h * 3600 - m * 60
        } else {
            h * 3600 + m * 60
        }
    }
}

#[derive(Debug)]
pub(crate) enum IsoOffset {
    Z,
    Numeric(IsoNumericOffset),
}

#[derive(Debug)]
pub(crate) struct IsoParsed {
    pub(crate) date: IsoDate,
    pub(crate) time: Option<IsoTime>,
    pub(crate) timezone_offset: Option<IsoOffset>,
    pub(crate) timezone_name: Option<String>,
    pub(crate) calendar: Option<String>,
}

type It<'a> = Peekable<Chars<'a>>;

fn parse_bracket(it: &mut It<'_>) -> Option<(String, bool)> {
    if *it.peek()? != '[' {
        return None;
    }
    it.next();
    let mut r = String::new();
    loop {
        let c = it.next()?;
        if c == ']' {
            break;
        }
        r.push(c);
    }
    if let Some(cal) = r.strip_prefix("u-ca=") {
        return Some((cal.to_string(), true));
    }
    Some((r, false))
}

fn parse_two_digit(it: &mut It<'_>) -> Option<u8> {
    let d1 = it.next()?;
    let d2 = it.next()?;
    if let Some(d1) = d1.to_digit(10) {
        if let Some(d2) = d2.to_digit(10) {
            return Some(d1 as u8 * 10 + d2 as u8);
        }
    }
    None
}

fn parse_two_digit_colon(it: &mut It<'_>, colon_optional: bool) -> Option<Vec<u8>> {
    let mut r = vec![parse_two_digit(it)?];
    loop {
        if let Some(':') = it.peek() {
            it.next();
            r.push(parse_two_digit(it)?);
        } else {
            if !colon_optional {
                break;
            }
            if it.peek().filter(|c| c.is_digit(10)).is_some() {
                r.push(parse_two_digit(it)?);
            } else {
                break;
            }
        }
    }
    Some(r)
}

fn parse_root(it: &mut It<'_>) -> Option<IsoParsed> {
    let date = parse_date(it)?;
    let mut time = None;
    let mut timezone_name = None;
    let mut timezone_offset = None;
    let mut calendar = None;
    loop {
        let c = match it.peek() {
            Some(c) => *c,
            None => {
                return Some(IsoParsed {
                    date,
                    time,
                    calendar,
                    timezone_name,
                    timezone_offset,
                })
            }
        };
        match c {
            'T' => {
                it.next();
                time = Some(parse_time(it, false)?);
            }
            'Z' => {
                it.next();
                timezone_offset = Some(IsoOffset::Z);
            }
            c if parse_sign(c).is_some() => {
                timezone_offset = Some(IsoOffset::Numeric(parse_numeric_timezone(it)?));
            }
            '[' => {
                let (content, is_cal) = parse_bracket(it)?;
                if is_cal {
                    calendar = Some(content);
                } else {
                    timezone_name = Some(content);
                }
            }
            _ => return None,
        }
    }
}

fn parse_numeric_timezone(it: &mut Peekable<Chars>) -> Option<IsoNumericOffset> {
    let c = it.next()?;
    let is_neg = parse_sign(c)?;
    let (hour, minute) = match parse_two_digit_colon(it, true)?.as_slice() {
        [h, m] => (*h, *m),
        [h] => (*h, 0),
        _ => return None,
    };
    Some(IsoNumericOffset {
        hour,
        is_neg,
        minute,
    })
}

pub(crate) fn parse_time(it: &mut It<'_>, colon_optional: bool) -> Option<IsoTime> {
    fn parse3(it: &mut It<'_>) -> u16 {
        let mut r = 0;
        for _ in 0..3 {
            r *= 10;
            if let Some(c) = it.peek() {
                if let Some(d) = c.to_digit(10) {
                    it.next();
                    r += d as u16;
                }
            }
        }
        r
    }
    let (hour, minute, second, has_sec) =
        match parse_two_digit_colon(it, colon_optional)?.as_slice() {
            [h] => (*h, 0, 0, false),
            [h, m] => (*h, *m, 0, false),
            [h, m, s] => (*h, *m, *s, true),
            _ => return None,
        };
    let (millisecond, microsecond, nanosecond) = if has_sec && it.peek() == Some(&'.') {
        it.next();
        let a = parse3(it);
        let b = parse3(it);
        let c = parse3(it);
        (a, b, c)
    } else {
        (0, 0, 0)
    };
    Some(IsoTime {
        hour,
        minute,
        second,
        millisecond,
        microsecond,
        nanosecond,
    })
}

fn parse_date(it: &mut Peekable<Chars>) -> Option<IsoDate> {
    let year = parse_num(it, 4)? as i32;
    eat_char(it, '-')?;
    let month = parse_num(it, 2)? as u8;
    eat_char(it, '-')?;
    let day = parse_num(it, 2)? as u16;
    Some(IsoDate { day, month, year })
}

fn eat_char(it: &mut It<'_>, c: char) -> Option<()> {
    if it.peek()? != &c {
        return None;
    }
    it.next();
    Some(())
}

fn parse_num(it: &mut It<'_>, mut cnt: usize) -> Option<u32> {
    if !it.peek()?.is_digit(10) {
        return None;
    }
    let mut result = 0;
    while let Some(x) = it.peek().and_then(|x| x.to_digit(10)) {
        if cnt == 0 {
            return Some(result);
        }
        result *= 10;
        result += x;
        it.next();
        cnt -= 1;
    }
    if cnt > 0 {
        return None;
    }
    Some(result)
}

/// returns Some(true) for - and u2212
pub(crate) fn parse_sign(c: char) -> Option<bool> {
    match c {
        '-' | '\u{2212}' => Some(true),
        '+' => Some(false),
        _ => None,
    }
}

pub(crate) fn parse(text: &str) -> Option<IsoParsed> {
    let mut chars = text.chars().peekable();
    parse_root(&mut chars)
}
