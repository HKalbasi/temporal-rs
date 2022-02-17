use std::str::FromStr;

use crate::{
    duration::SignedDuration,
    iso::{self, IsoDate, IsoOffset},
    timezone::{TimeZone, TimeZoneProtocol},
    Calendar, CalendarProtocol, PlainDate,
};

pub struct ZonedDateTime<T: TimeZoneProtocol = TimeZone, C: CalendarProtocol = Calendar> {
    epoch: SignedDuration,
    calendar: C,
    timezone: T,
}

impl<T: TimeZoneProtocol, C: CalendarProtocol> From<ZonedDateTime<T, C>> for PlainDate<C> {
    fn from(z: ZonedDateTime<T, C>) -> Self {
        let IsoDate {
            year: iso_year,
            month: iso_month,
            day: iso_day,
        } = z.iso_date();
        Self {
            calendar: z.calendar,
            iso_day,
            iso_month,
            iso_year,
        }
    }
}

impl<T: TimeZoneProtocol, C: CalendarProtocol> ZonedDateTime<T, C> {
    pub(crate) fn iso_date(&self) -> IsoDate {
        let secs = self.epoch.as_secs() + self.timezone.get_second_offset(self.epoch.as_secs());
        IsoDate::from_epoch_second(secs)
    }

    pub fn year(&self) -> i32 {
        let IsoDate { year, month, day } = self.iso_date();
        self.calendar.year(year, month, day)
    }
    pub fn month(&self) -> u32 {
        let IsoDate { year, month, day } = self.iso_date();
        self.calendar.month(year, month, day)
    }
    pub fn day(&self) -> u32 {
        let IsoDate { year, month, day } = self.iso_date();
        self.calendar.day(year, month, day)
    }
    pub fn hour(&self) -> u8 {
        let secs = self.epoch.as_secs() + self.timezone.get_second_offset(self.epoch.as_secs());
        secs.div_euclid(3600).rem_euclid(24) as u8
    }
    pub fn minute(&self) -> u8 {
        let secs = self.epoch.as_secs() + self.timezone.get_second_offset(self.epoch.as_secs());
        secs.div_euclid(60).rem_euclid(60) as u8
    }
}

#[non_exhaustive]
#[derive(Debug)]
pub enum ZonedDateTimeParseError {
    MalformedIsoString,
    UnknownTimeZone(String),
    UnknownCalendar(String),
    MissingTimezone,
    NonUniqueTime,
    WrongOffset,
}

impl FromStr for ZonedDateTime {
    type Err = ZonedDateTimeParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use ZonedDateTimeParseError::*;
        let i = iso::parse(s).ok_or(MalformedIsoString)?;
        let tz_name = i.timezone_name.ok_or(MissingTimezone)?;
        let tz = TimeZone::from_str(&tz_name).map_err(|_| UnknownTimeZone(tz_name))?;
        let calendar = if let Some(c) = i.calendar {
            c.parse().map_err(|_| UnknownCalendar(c))?
        } else {
            Calendar::Iso8601
        };
        let time_secs = if let Some(x) = i.time {
            x.to_second().into()
        } else {
            0
        };
        let secs = i.date.to_epoch_second() + time_secs;
        let real_secs = match i.timezone_offset {
            Some(IsoOffset::Numeric(n)) => {
                let offset_secs = n.to_seconds().into();
                let x = secs - offset_secs;
                let offset_tz = tz.get_second_offset(x);
                if offset_tz != offset_secs {
                    return Err(WrongOffset);
                }
                x
            }
            Some(IsoOffset::Z) => {
                secs
            }
            None => {
                let d = tz.get_possible_seconds(i.date, i.time.unwrap_or_default());
                if d.len() != 1 {
                    return Err(NonUniqueTime);
                }
                d[0]
            }
        };
        Ok(Self {
            calendar,
            timezone: tz,
            epoch: SignedDuration::from_secs(real_secs),
        })
    }
}
