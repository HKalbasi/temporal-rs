mod calendar;
mod duration;
mod iso;
mod plain;
mod timezone;

pub use calendar::{Calendar, CalendarProtocol, Era};
pub use plain::PlainDate;

#[cfg(test)]
mod tests {
    use crate::{Calendar, PlainDate};

    #[test]
    fn parse_simple() {
        let result: PlainDate = "2022-02-02".parse().unwrap();
        assert_eq!(result.year(), 2022);
    }

    #[test]
    fn from_ymd_constraint() {
        let result = PlainDate::from_ymd(2000, 13, 2, Calendar::Iso8601).constrain();
        assert_eq!(result.month(), 12);
    }
}
