mod calendar;
mod duration;
mod iso;
mod plain;
mod timezone;
mod zoned;

pub use calendar::{Calendar, CalendarProtocol, Era};
pub use plain::PlainDate;
pub use zoned::ZonedDateTime;
pub use timezone::{TimeZone, TimeZoneProtocol};

#[cfg(test)]
mod tests {
    use std::str::FromStr;

    use crate::{
        zoned::{ZonedDateTime, ZonedDateTimeParseError},
        Calendar, PlainDate,
    };

    #[test]
    fn parse_plain_date_simple() {
        let result: PlainDate = "2022-02-02".parse().unwrap();
        assert_eq!(result.year(), 2022);
    }

    #[test]
    fn parse_zoned_simple() {
        let result: ZonedDateTime = "2022-09-01T00:00Z[Asia/TehrAn]".parse().unwrap();
        assert_eq!(result.year(), 2022);
        assert_eq!(result.month(), 9);
        assert_eq!(result.day(), 1);
        assert_eq!(result.hour(), 4);
        assert_eq!(result.minute(), 30);
        let result2: ZonedDateTime = "2122-04-01T07:12Z[+03:00]".parse().unwrap();
        assert_eq!(result2.year(), 2122);
        assert_eq!(result2.month(), 4);
        assert_eq!(result2.day(), 1);
        assert_eq!(result2.hour(), 10);
        assert_eq!(result2.minute(), 12);
        let result3: ZonedDateTime = "1853-04-01T00:00Z[UTC]".parse().unwrap();
        assert_eq!(result3.year(), 1853);
        assert_eq!(result3.month(), 4);
        assert_eq!(result3.day(), 1);
        assert_eq!(result3.hour(), 0);
        assert!(matches!(
            ZonedDateTime::from_str("2022-09-01T00:00+04[+04:30]"),
            Err(ZonedDateTimeParseError::WrongOffset),
        ));
    }

    #[test]
    fn from_ymd_constraint() {
        let result = PlainDate::from_ymd(2000, 13, 2, Calendar::Iso8601).constrain();
        assert_eq!(result.month(), 12);
    }
}
