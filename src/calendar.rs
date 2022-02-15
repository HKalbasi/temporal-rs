use std::str::FromStr;

use crate::{duration::NominalDuration, iso::IsoDate};

#[derive(Debug, Clone, Copy)]
pub enum Calendar {
    Iso8601,
    Persian,
}

impl FromStr for Calendar {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(match s {
            "iso8601" => Self::Iso8601,
            "persian" => Self::Persian,
            _ => return Err(()),
        })
    }
}

pub struct Era {
    pub name: String,
    pub year: u32,
}

pub enum FromYMDResult {
    Normal(IsoDate),
    OverflowConstrained(IsoDate),
}

pub trait CalendarProtocol {
    fn id(&self) -> String;
    fn era(&self, iso_year: i32, iso_month: u8, iso_day: u16) -> Option<Era>;
    fn year(&self, iso_year: i32, iso_month: u8, iso_day: u16) -> i32;
    fn month(&self, iso_year: i32, iso_month: u8, iso_day: u16) -> u32;
    fn month_code(&self, iso_year: i32, iso_month: u8, iso_day: u16) -> String;
    fn day(&self, iso_year: i32, iso_month: u8, iso_day: u16) -> u32;
    fn day_of_week(&self, iso_year: i32, iso_month: u8, iso_day: u16) -> u32;
    fn day_of_year(&self, iso_year: i32, iso_month: u8, iso_day: u16) -> u32;
    fn week_of_year(&self, iso_year: i32, iso_month: u8, iso_day: u16) -> u32;
    fn days_in_week(&self, iso_year: i32, iso_month: u8, iso_day: u16) -> u32;
    fn days_in_month(&self, iso_year: i32, iso_month: u8, iso_day: u16) -> u32;
    fn days_in_year(&self, iso_year: i32, iso_month: u8, iso_day: u16) -> u32;
    fn months_in_year(&self, iso_year: i32, iso_month: u8, iso_day: u16) -> u32;
    fn in_leap_year(&self, iso_year: i32, iso_month: u8, iso_day: u16) -> bool;
    fn from_ymd(&self, year: i32, month: u32, day: u32) -> FromYMDResult;
    fn date_add(
        &self,
        iso_year: i32,
        iso_month: u8,
        iso_day: u16,
        dur: NominalDuration,
    ) -> FromYMDResult;
}

struct IsoCalendar;

mod impls;

impl Calendar {
    fn to_trait_obj(self) -> Box<dyn CalendarProtocol> {
        match self {
            Calendar::Iso8601 => Box::new(IsoCalendar),
            Calendar::Persian => todo!(),
        }
    }
}

#[allow(unused_variables)]
impl CalendarProtocol for Calendar {
    fn id(&self) -> String {
        self.to_trait_obj().id()
    }

    fn era(&self, iso_year: i32, iso_month: u8, iso_day: u16) -> Option<Era> {
        self.to_trait_obj().era(iso_year, iso_month, iso_day)
    }

    fn year(&self, iso_year: i32, iso_month: u8, iso_day: u16) -> i32 {
        self.to_trait_obj().year(iso_year, iso_month, iso_day)
    }

    fn month(&self, iso_year: i32, iso_month: u8, iso_day: u16) -> u32 {
        self.to_trait_obj().month(iso_year, iso_month, iso_day)
    }

    fn month_code(&self, iso_year: i32, iso_month: u8, iso_day: u16) -> String {
        self.to_trait_obj().month_code(iso_year, iso_month, iso_day)
    }

    fn day(&self, iso_year: i32, iso_month: u8, iso_day: u16) -> u32 {
        self.to_trait_obj().day(iso_year, iso_month, iso_day)
    }

    fn day_of_week(&self, iso_year: i32, iso_month: u8, iso_day: u16) -> u32 {
        self.to_trait_obj()
            .day_of_week(iso_year, iso_month, iso_day)
    }

    fn day_of_year(&self, iso_year: i32, iso_month: u8, iso_day: u16) -> u32 {
        self.to_trait_obj()
            .day_of_year(iso_year, iso_month, iso_day)
    }

    fn week_of_year(&self, iso_year: i32, iso_month: u8, iso_day: u16) -> u32 {
        self.to_trait_obj()
            .week_of_year(iso_year, iso_month, iso_day)
    }

    fn days_in_week(&self, iso_year: i32, iso_month: u8, iso_day: u16) -> u32 {
        self.to_trait_obj()
            .days_in_week(iso_year, iso_month, iso_day)
    }

    fn days_in_month(&self, iso_year: i32, iso_month: u8, iso_day: u16) -> u32 {
        self.to_trait_obj()
            .days_in_month(iso_year, iso_month, iso_day)
    }

    fn days_in_year(&self, iso_year: i32, iso_month: u8, iso_day: u16) -> u32 {
        self.to_trait_obj()
            .days_in_year(iso_year, iso_month, iso_day)
    }

    fn months_in_year(&self, iso_year: i32, iso_month: u8, iso_day: u16) -> u32 {
        self.to_trait_obj()
            .months_in_year(iso_year, iso_month, iso_day)
    }

    fn in_leap_year(&self, iso_year: i32, iso_month: u8, iso_day: u16) -> bool {
        self.to_trait_obj()
            .in_leap_year(iso_year, iso_month, iso_day)
    }

    fn from_ymd(&self, year: i32, month: u32, day: u32) -> FromYMDResult {
        self.to_trait_obj().from_ymd(year, month, day)
    }

    fn date_add(
        &self,
        iso_year: i32,
        iso_month: u8,
        iso_day: u16,
        dur: NominalDuration,
    ) -> FromYMDResult {
        self.to_trait_obj()
            .date_add(iso_year, iso_month, iso_day, dur)
    }
}
