use std::time::{SystemTime, UNIX_EPOCH};

use crate::iso;

#[derive(Default)]
pub struct NominalDuration {
    is_negative: bool,
    years: u32,
    months: u32,
    weeks: u32,
    days: u32,
    hours: u32,
    minutes: u32,
    seconds: u32,
    milli_seconds: u32,
    micro_seconds: u32,
    nano_seconds: u32,
}

impl NominalDuration {
    fn signum(&self) -> i32 {
        if self.is_negative {
            -1
        } else {
            1
        }
    }

    pub fn from_years(years: i32) -> Self {
        Self {
            years: years.abs().try_into().unwrap(),
            ..Self::default()
        }
    }

    pub fn years(&self) -> i32 {
        self.years as i32 * self.signum()
    }
    pub fn months(&self) -> i32 {
        self.months as i32 * self.signum()
    }
    pub fn weeks(&self) -> i32 {
        self.weeks as i32 * self.signum()
    }
    pub fn days(&self) -> i32 {
        self.days as i32 * self.signum()
    }
}

pub struct SignedDuration {
    secs: i64,
    nanos: u32, // Always 0 <= nanos < NANOS_PER_SEC
}

impl SignedDuration {
    pub fn now() -> Self {
        let now = SystemTime::now();
        Self::from_system_time_since_unix(now)
    }

    pub fn new(mut secs: i64, nanos: i32) -> Self {
        secs += nanos.div_euclid(1000_000_000) as i64;
        Self {
            secs,
            nanos: nanos.rem_euclid(1000_000_000) as u32,
        }
    }

    pub fn from_iso_string(s: &str) -> Option<Self> {
        let i = iso::parse(s)?;
        let offset_secs = match i.timezone_offset? {
            iso::IsoOffset::Z => 0,
            iso::IsoOffset::Numeric(x) => x.to_seconds() as i64,
        };
        let time_secs = if let Some(x) = i.time {
            x.to_second().into()
        } else {
            0
        };
        let secs = i.date.to_epoch_second() + time_secs + offset_secs;
        Some(Self::new(secs, 0))
    }

    pub(crate) fn from_system_time_since_unix(s: SystemTime) -> Self {
        match s.duration_since(UNIX_EPOCH) {
            Ok(d) => SignedDuration {
                secs: d.as_secs() as i64,
                nanos: d.subsec_nanos(),
            },
            Err(e) => {
                let d = e.duration();
                SignedDuration {
                    secs: -(d.as_secs() as i64),
                    nanos: d.subsec_nanos(),
                }
            }
        }
    }

    pub fn from_secs(secs: i64) -> Self {
        Self { secs, nanos: 0 }
    }

    pub fn as_secs(&self) -> i64 {
        self.secs
    }
}
