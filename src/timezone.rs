use std::time::Duration;

use crate::iso::{IsoDate, IsoTime};

pub trait TimezoneProtocol {
    fn id(&self) -> String;
    fn get_nanosecond_offset(&self, duration_since_epoch: Duration) -> u64;
    fn get_possible_instants(&self, date: IsoDate, time: IsoTime) -> Vec<Duration>;
}