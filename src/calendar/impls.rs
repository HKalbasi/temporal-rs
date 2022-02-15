use super::*;

#[allow(unused)]
impl CalendarProtocol for IsoCalendar {
    fn id(&self) -> String {
        "iso8601".to_string()
    }

    fn era(&self, iso_year: i32, iso_month: u8, iso_day: u16) -> Option<Era> {
        None
    }

    fn year(&self, iso_year: i32, iso_month: u8, iso_day: u16) -> i32 {
        iso_year
    }

    fn month(&self, iso_year: i32, iso_month: u8, iso_day: u16) -> u32 {
        iso_month.into()
    }

    fn month_code(&self, iso_year: i32, iso_month: u8, iso_day: u16) -> String {
        format!("M{}", iso_month)
    }

    fn day(&self, iso_year: i32, iso_month: u8, iso_day: u16) -> u32 {
        iso_day.into()
    }

    fn day_of_week(&self, iso_year: i32, iso_month: u8, iso_day: u16) -> u32 {
        todo!()
    }

    fn day_of_year(&self, iso_year: i32, iso_month: u8, iso_day: u16) -> u32 {
        let mut r = iso_day.into();
        for i in 1..iso_month {
            r += self.days_in_month(iso_year, i, 1);
        }
        r
    }

    fn week_of_year(&self, iso_year: i32, iso_month: u8, iso_day: u16) -> u32 {
        todo!()
    }

    fn days_in_week(&self, iso_year: i32, iso_month: u8, iso_day: u16) -> u32 {
        7
    }

    fn days_in_month(&self, iso_year: i32, iso_month: u8, iso_day: u16) -> u32 {
        const MONTHS: [u8; 12] = [31, 28, 31, 30, 31, 30, 31, 31, 30, 31, 30, 31];
        if iso_month == 2 && self.in_leap_year(iso_year, iso_month, iso_day) {
            29
        } else {
            MONTHS[usize::from(iso_month) - 1].into()
        }
    }

    fn days_in_year(&self, iso_year: i32, iso_month: u8, iso_day: u16) -> u32 {
        if self.in_leap_year(iso_year, iso_month, iso_day) {
            366
        } else {
            365
        }
    }

    fn months_in_year(&self, iso_year: i32, iso_month: u8, iso_day: u16) -> u32 {
        12
    }

    fn in_leap_year(&self, iso_year: i32, iso_month: u8, iso_day: u16) -> bool {
        iso_year % 4 == 0 && (iso_year % 100 != 0 || iso_year % 400 == 0)
    }

    fn from_ymd(&self, year: i32, month: u32, day: u32) -> FromYMDResult {
        if month == 0 || day == 0 {
            panic!("month and day should be positive");
        }
        if month > 12 {
            FromYMDResult::OverflowConstrained(IsoDate {
                year,
                month: 12,
                day: if day > 31 {
                    31
                } else {
                    day.try_into().unwrap()
                },
            })
        } else {
            let month = month.try_into().unwrap();
            let l = self.days_in_month(year, month, 1);
            if day > l {
                FromYMDResult::OverflowConstrained(IsoDate {
                    year,
                    month,
                    day: l.try_into().unwrap(),
                })
            } else {
                let day = day.try_into().unwrap();
                FromYMDResult::OverflowConstrained(IsoDate { year, month, day })
            }
        }
    }

    fn date_add(
        &self,
        iso_year: i32,
        iso_month: u8,
        iso_day: u16,
        dur: NominalDuration,
    ) -> FromYMDResult {
        fn balance_year_month(year: i32, month: i32) -> (i32, u8) {
            (
                year + (month - 1).div_euclid(12),
                (month - 1).rem_euclid(12).try_into().unwrap(),
            )
        }
        let (mut r_year, mut r_month) = balance_year_month(
            iso_year + dur.years(),
            iso_month as i32 + dur.months(),
        );
        let mut need_constrain = false;
        let mut r_day: i32 = iso_day.into();
        if self.days_in_month(r_year, r_month, 1) > iso_day.into() {
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
        }
    }
}
