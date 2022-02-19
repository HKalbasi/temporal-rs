/// An Offset that applies for a period of time
///
/// For example, [`::US::Eastern`] is composed of at least two
/// `FixedTimespan`s: `EST` and `EDT`, that are variously in effect.
#[derive(Copy, Clone, PartialEq, Eq)]
pub struct FixedTimespan {
    /// The base offset from UTC; this usually doesn't change unless the government changes something
    pub utc_offset: i32,
    /// The additional offset from UTC for this timespan; typically for daylight saving time
    pub dst_offset: i32,
    /// The name of this timezone, for example the difference between `EDT`/`EST`
    pub name: &'static str,
}

impl FixedTimespan {
    pub(super) fn second_offset(&self) -> i64 {
        (self.utc_offset + self.dst_offset) as i64
    }
}

#[derive(Copy, Clone)]
pub struct FixedTimespanSet {
    pub first: FixedTimespan,
    pub rest: &'static [(i64, FixedTimespan)],
}

impl FixedTimespanSet {
    pub(super) fn select_with_sec(&self, sec: i64) -> FixedTimespan {
        if self.rest.is_empty() || sec < self.rest[0].0 {
            return self.first;
        }
        let mut l = 0;
        let mut r = self.rest.len();
        while r - l > 1 {
            let mid = (l + r) / 2;
            if sec < self.rest[mid].0 {
                r = mid;
            } else {
                l = mid;
            }
        }
        self.rest[l].1
    }
}

pub trait TimeSpans {
    fn timespans(&self) -> FixedTimespanSet;
}
