#[derive(Debug, PartialEq)]
pub struct Clock {
    hours: i32,
    minutes: i32,
}

fn div_euc(dividend: i32, divisor: i32) -> i32 {
    let q = dividend / divisor;
    if dividend % divisor < 0 {
        return q - 1;
    }
    q
}

fn mod_euc(dividend: i32, divisor: i32) -> i32 {
    let result = dividend % divisor;
    if result < 0 {
        return result + divisor;
    }
    result
}

impl Clock {
    pub fn new(hours: i32, minutes: i32) -> Self {
        Clock {
            hours: mod_euc(hours + div_euc(minutes, 60), 24),
            minutes: mod_euc(minutes, 60),
        }
    }

    pub fn add_minutes(self, minutes: i32) -> Self {
        Clock::new(self.hours, self.minutes + minutes)
    }
}

impl std::fmt::Display for Clock {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{:02}:{:02}", self.hours, self.minutes)
    }
}
