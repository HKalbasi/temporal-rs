use std::{str::FromStr, fmt::Debug};

use crate::iso::{IsoDate, IsoTime, self, parse_time, parse_sign};

mod iana_generated;
mod timezone_impl;

pub trait TimeZoneProtocol {
    fn id(&self) -> String;
    fn get_second_offset(&self, seconds_since_epoch: i64) -> i64;
    fn get_possible_seconds(&self, date: IsoDate, time: IsoTime) -> Vec<i64>;
}

use iana_generated::Tz;

use self::timezone_impl::TimeSpans;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum TimezoneInner {
    Tz(Tz),
    Fixed(i32),
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub struct TimeZone(TimezoneInner);

impl TimeZone {
    const UTC: Self = Self(TimezoneInner::Tz(Tz::UTC));

    pub fn extract_from_iso_date(iso: &str) -> Result<Self, TimeZoneParseError> {
        use TimeZoneParseError::*;
        let i = iso::parse(iso).ok_or(InvalidIsoString)?;
        if let Some(x) = i.timezone_name {
            return x.parse();
        }
        Ok(match i.timezone_offset.ok_or(InvalidIsoString)? {
            iso::IsoOffset::Z => Self::UTC,
            iso::IsoOffset::Numeric(x) => Self(TimezoneInner::Fixed(x.to_seconds())),
        })
    }
}

impl Debug for TimeZone {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_tuple("TimeZone").field(&self.id()).finish()
    }
}

#[derive(Debug, PartialEq, Eq)]
pub enum TimeZoneParseError {
    InvalidTimeZone,
    InvalidIsoString,
    OutOfRange,
    SubSecondOffset(i64),
}

impl TimeZoneParseError {
    pub fn is_sub_second(&self) -> Option<i64> {
        match self {
            Self::SubSecondOffset(x) => Some(*x),
            _ => None
        }
    }
}

impl FromStr for TimeZone {
    type Err = TimeZoneParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use TimeZoneParseError::*;
        let mut it = s.chars().peekable();
        if let Ok(t) = Tz::from_str_insensitive(s) {
            return Ok(Self(TimezoneInner::Tz(t)));
        }
        if let Some(c) = it.next() {
            if let Some(is_neg) = parse_sign(c) {
                if let Some(t) = parse_time(&mut it, true) {
                    if it.next().is_some() {
                        return Err(InvalidIsoString);
                    }
                    if t.has_nanosecond() {
                        let t = t.to_nanosecond();
                        let r = if is_neg { -t } else { t };
                        return Err(SubSecondOffset(r));
                    }
                    let t = t.to_second();
                    let r = if is_neg { -t } else { t };
                    return Ok(Self(TimezoneInner::Fixed(r)));
                }
                return Err(InvalidIsoString);
            }
        }
        Err(InvalidTimeZone)
    }
}

impl TimeZoneProtocol for TimeZone {
    fn id(&self) -> String {
        match &self.0 {
            TimezoneInner::Tz(x) => x.name().to_string(),
            TimezoneInner::Fixed(x) => {
                let sign = if *x < 0 { '-' } else { '+' };
                let x = x.abs();
                let secs = x % 60;
                let x = x / 60;
                let mins = x % 60;
                let hours = x / 60;
                if secs == 0 {
                    format!("{}{:02}:{:02}", sign, hours, mins)
                } else {
                    format!("{}{:02}:{:02}:{:02}", sign, hours, mins, secs)    
                }
            },
        }
    }

    fn get_second_offset(&self, seconds_since_epoch: i64) -> i64 {
        match &self.0 {
            TimezoneInner::Tz(x) => x
                .timespans()
                .select_with_sec(seconds_since_epoch)
                .second_offset(),
            TimezoneInner::Fixed(x) => (*x).into(),
        }
    }

    fn get_possible_seconds(&self, date: IsoDate, time: IsoTime) -> Vec<i64> {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use crate::{TimeZone, TimeZoneProtocol, timezone::TimeZoneParseError};

    #[test]
    fn simple_parse() {
        let tz: TimeZone = "+01:00".parse().unwrap();
        assert_eq!(tz.get_second_offset(0), 3600);
    }

    #[test]
    fn parse_without_colon() {
        let tz: TimeZone = "+0330".parse().unwrap();
        assert_eq!(tz.get_second_offset(0), 12600);
    }

    #[test]
    fn parse_sub_second() {
        let tz: Result<TimeZone, TimeZoneParseError> = "-03:30:00.000000001".parse();
        assert_eq!(tz, Err(TimeZoneParseError::SubSecondOffset(-12600_000000001)));
    }
}
