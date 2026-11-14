use crate::Time;
use std::ops::{Add, AddAssign, Sub, SubAssign};

#[derive(Copy, Clone, Debug)]
pub struct SignedTime {
    is_positive: bool,
    time: Time,
}

impl SignedTime {
    pub const ZERO: SignedTime = SignedTime::new(true, Time::ZERO);

    pub const fn new(is_positive: bool, unsigned_time: Time) -> SignedTime {
        Self {
            is_positive,
            time: unsigned_time,
        }
    }

    pub const fn as_unsigned_time(&self) -> Time {
        self.time
    }

    pub const fn is_positive(&self) -> bool {
        self.is_positive
    }
}

impl Add for SignedTime {
    type Output = Self;

    fn add(mut self, other: Self) -> Self {
        if self.is_positive == other.is_positive {
            self.time += other.time;
        } else if self.time > other.time {
            self.time -= other.time;
        } else if self.time < other.time {
            self.is_positive = !self.is_positive;
            self.time = other.time - self.time;
        } else {
            self.is_positive = true;
            self.time = Time::ZERO;
        }
        self
    }
}
impl AddAssign for SignedTime {
    fn add_assign(&mut self, other: Self) {
        *self = *self + other;
    }
}

impl Sub for SignedTime {
    type Output = Self;

    fn sub(self, mut other: Self) -> Self::Output {
        // A - B  =>  A + (-B)
        other.is_positive = !other.is_positive;
        self + other
    }
}
impl SubAssign for SignedTime {
    fn sub_assign(&mut self, other: Self) {
        *self = *self - other;
    }
}

impl Add<Time> for SignedTime {
    type Output = Self;

    fn add(self, unsigned_time: Time) -> Self {
        self.add(SignedTime::new(true, unsigned_time))
    }
}
impl Sub<Time> for SignedTime {
    type Output = Self;

    fn sub(self, unsigned_time: Time) -> Self {
        self.sub(SignedTime::new(true, unsigned_time))
    }
}
