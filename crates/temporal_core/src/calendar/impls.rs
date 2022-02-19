use std::cmp::{max, min};

use super::*;

#[allow(unused)]
impl CalendarProtocol for IsoCalendar {
    fn id(&self) -> String {
        "iso8601".to_string()
    }

    fn era(&self, iso_date: IsoDate) -> Option<Era> {
        None
    }

    fn year(&self, iso_date: IsoDate) -> i32 {
        iso_date.year()
    }

    fn month(&self, iso_date: IsoDate) -> u32 {
        iso_date.month().into()
    }

    fn month_code(&self, iso_date: IsoDate) -> String {
        format!("M{}", iso_date.month())
    }

    fn day(&self, iso_date: IsoDate) -> u32 {
        iso_date.day().into()
    }

    fn day_of_week(&self, iso_date: IsoDate) -> u32 {
        iso_date.to_icu_date().day_of_week() as u32
    }

    fn day_of_year(&self, iso_date: IsoDate) -> u32 {
        iso_date.to_icu_date().day_of_year_info().day_of_year
    }

    fn week_of_year(&self, iso_date: IsoDate) -> u32 {
        todo!()
    }

    fn days_in_week(&self, iso_date: IsoDate) -> u32 {
        7
    }

    fn days_in_month(&self, iso_date: IsoDate) -> u32 {
        iso_date.to_icu_date().days_in_month().into()
    }

    fn days_in_year(&self, iso_date: IsoDate) -> u32 {
        iso_date.to_icu_date().days_in_year()
    }

    fn months_in_year(&self, iso_date: IsoDate) -> u32 {
        12
    }

    fn in_leap_year(&self, iso_date: IsoDate) -> bool {
        iso_date.to_icu_date().days_in_year() == 366
    }

    fn from_ymd(&self, year: i32, month: u32, day: u32) -> FromYMDResult {
        if let Ok(month) = month.try_into() {
            if let Ok(day) = day.try_into() {
                if let Some(x) = IsoDate::new(year, month, day) {
                    return FromYMDResult::Normal(x);
                }
            }
        }
        let year = max(min(year, IsoDate::MAX_YEAR), IsoDate::MIN_YEAR);
        let month = max(min(month, 12), 1) as u8;
        let max_day = self.days_in_month(IsoDate::new_unchecked(year, month, 1));
        let day = max(min(day, max_day), 1) as u16;
        FromYMDResult::OverflowConstrained(IsoDate::new_unchecked(year, month, day))
    }

    fn date_add(&self, iso_date: IsoDate, dur: NominalDuration) -> FromYMDResult {
        /*fn balance_year_month(year: i32, month: i32) -> (i32, u8) {
            (
                year + (month - 1).div_euclid(12),
                (month - 1).rem_euclid(12).try_into().unwrap(),
            )
        }
        let (mut r_year, mut r_month) = balance_year_month(
            iso_date.year + dur.years(),
            iso_date.month() as i32 + dur.months(),
        );
        let mut need_constrain = false;
        let mut r_day: i32 = iso_date.day().into();
        if self.days_in_month(r_year, r_month, 1) > iso_date.day().into() {
            need_constrain = true;
            r_day = self.days_in_month(r_year, r_month, 1).try_into().unwrap();
        }
        r_day += 7 * dur.weeks();
        r_day += dur.days();
        loop {
            let dim = self.days_in_month(r_year, r_month, 1).try_into().unwrap();
            if r_day <= dim {
                break;
            }
            r_day -= dim;
            r_month += 1;
            if r_month == 13 {
                r_month = 1;
                r_year += 1;
            }
        }
        while r_day < 1 {
            r_month -= 1;
            if r_month == 0 {
                r_month = 12;
                r_year -= 1;
            }
            r_day += self.days_in_month(r_year, r_month, 1) as i32;
        }
        let iso_date = IsoDate {
            day: r_day.try_into().unwrap(),
            month: r_month,
            year: r_year,
        };
        if need_constrain {
            FromYMDResult::OverflowConstrained(iso_date)
        } else {
            FromYMDResult::Normal(iso_date)
        }*/
        todo!()
    }
}
