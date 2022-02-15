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
        if self.is_negative { -1 } else { 1 }
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
