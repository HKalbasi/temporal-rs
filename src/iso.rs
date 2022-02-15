use std::{iter::Peekable, str::Chars};

pub struct IsoDate {
    pub year: i32,
    pub month: u8,
    pub day: u16,
}

pub struct IsoTime {
    pub hour: u8,
    pub minute: u8,
    pub second: u8,
    pub millisecond: u16,
    pub microsecond: u16,
    pub nanosecond: u16,
}

pub(crate) struct IsoParsed {
    pub(crate) date: IsoDate,
    pub(crate) time: Option<IsoTime>,
}

type It<'a> = Peekable<Chars<'a>>;

fn parse_root(it: &mut It<'_>) -> Option<IsoParsed> {
    let date = parse_date(it)?;
    let time = parse_time(it);
    if it.peek().is_some() {
        return None;
    }
    Some(IsoParsed { date, time })
}

fn parse_time(it: &mut Peekable<Chars>) -> Option<IsoTime> {
    None
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

pub(crate) fn parse(text: &str) -> Option<IsoParsed> {
    let mut chars = text.chars().peekable();
    parse_root(&mut chars)
}
