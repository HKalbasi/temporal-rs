use std::str::FromStr;

use crate::iso;

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
            FromYMDResult::Normal(x) | FromYMDResult::OverflowConstrained(x) => PlainDate {
                calendar: self.1,
                iso_day: x.day,
                iso_month: x.month,
                iso_year: x.year,
            },
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

    pub fn iso_year(&self) -> i32 {
        self.iso_year
    }
    pub fn iso_month(&self) -> u8 {
        self.iso_month
    }
    pub fn iso_day(&self) -> u16 {
        self.iso_day
    }

    pub fn year(&self) -> i32 {
        self.calendar
            .year(self.iso_year, self.iso_month, self.iso_day)
    }

    pub fn month(&self) -> u32 {
        self.calendar
            .month(self.iso_year, self.iso_month, self.iso_day)
    }
}

impl FromStr for PlainDate {
    type Err = ();
    fn from_str(x: &str) -> Result<Self, ()> {
        let i = iso::parse(x).ok_or(())?.date;
        Ok(Self {
            calendar: Calendar::Iso8601,
            iso_year: i.year,
            iso_month: i.month,
            iso_day: i.day,
        })
    }
}
