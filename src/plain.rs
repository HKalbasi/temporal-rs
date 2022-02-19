use std::str::FromStr;

use crate::iso::{self, IsoDate};

use super::calendar::*;

#[derive(Clone, Copy)]
pub struct PlainDate<C: CalendarProtocol = Calendar> {
    pub(crate) calendar: C,
    pub(crate) iso_year: i32,
    pub(crate) iso_month: u8,
    pub(crate) iso_day: u16,
}

const _: () = assert!(core::mem::size_of::<PlainDate>() == 8);

pub struct MaybeOutOfRangePlainDate<C: CalendarProtocol>(FromYMDResult, C);

impl<C: CalendarProtocol> MaybeOutOfRangePlainDate<C> {
    pub fn constrain(self) -> PlainDate<C> {
        match self.0 {
            FromYMDResult::Normal(x) | FromYMDResult::OverflowConstrained(x) => {
                PlainDate::from_iso_date(x, self.1)
            }
        }
    }
}

impl<C: CalendarProtocol + Copy> PlainDate<C> {
    pub fn calendar(self) -> C {
        self.calendar
    }
}

impl<C: CalendarProtocol> PlainDate<C> {
    pub fn from_ymd(year: i32, month: u32, day: u32, calendar: C) -> MaybeOutOfRangePlainDate<C> {
        MaybeOutOfRangePlainDate(calendar.from_ymd(year, month, day), calendar)
    }

    pub fn from_iso_date(iso_date: IsoDate, calendar: C) -> Self {
        Self {
            calendar,
            iso_day: iso_date.day(),
            iso_month: iso_date.month(),
            iso_year: iso_date.year(),
        }
    }

    pub fn iso_date(&self) -> IsoDate {
        IsoDate::new_unchecked(self.iso_year, self.iso_month, self.iso_day)
    }

    pub fn year(&self) -> i32 {
        self.calendar.year(self.iso_date())
    }

    pub fn month(&self) -> u32 {
        self.calendar.month(self.iso_date())
    }
}

impl FromStr for PlainDate {
    type Err = ();
    fn from_str(x: &str) -> Result<Self, ()> {
        let i = iso::parse(x).ok_or(())?.date;
        Ok(Self::from_iso_date(i, Calendar::Iso8601))
    }
}
