use temporal_core::PlainDate;

/// WIP
pub enum Formatter {}

pub trait ToLocaleString {
    fn to_locale_string(f: &Formatter) -> String;
}

impl ToLocaleString for PlainDate {
    fn to_locale_string(_: &Formatter) -> String {
        todo!()
    }
}