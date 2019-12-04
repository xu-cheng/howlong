extern crate thiserror;

use core::ops::{Add, AddAssign, Sub, SubAssign};

#[derive(thiserror::Error, Debug)]
pub enum Error {
    /// Error during the system call.
    #[error("failed to call `{0}` [errno: {1}]")]
    SystemError(&'static str, i32),
    /// Error if `sysconf(_SC_CLK_TCK)` returns a too large value
    #[error("the clock frequence is too high.")]
    ClkFreqTooHigh,
}

pub type Result<T> = core::result::Result<T, Error>;

pub use core::time::Duration;

/// A point in time.
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct TimePoint(pub(crate) Duration);

impl Sub for TimePoint {
    type Output = Duration;

    #[inline(always)]
    fn sub(self, other: Self) -> Self::Output {
        self.0 - other.0
    }
}

/// Like [`TimePoint`] but captures real, user-CPU, and system-CPU process times.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ProcessTimePoint {
    pub(crate) real: Duration,
    pub(crate) user: Duration,
    pub(crate) system: Duration,
}

impl Sub for ProcessTimePoint {
    type Output = ProcessDuration;

    #[inline(always)]
    fn sub(self, other: Self) -> Self::Output {
        ProcessDuration {
            real: self.real - other.real,
            user: self.user - other.user,
            system: self.system - other.system,
        }
    }
}

/// Like [`Duration`] but captures real, user-CPU, and system-CPU process times.
#[derive(Clone, Copy, Debug, Default)]
pub struct ProcessDuration {
    pub real: Duration,
    pub user: Duration,
    pub system: Duration,
}

impl Add for ProcessDuration {
    type Output = Self;

    #[inline(always)]
    fn add(self, rhs: Self) -> Self::Output {
        ProcessDuration {
            real: self.real + rhs.real,
            user: self.user + rhs.user,
            system: self.system + rhs.system,
        }
    }
}

impl AddAssign for ProcessDuration {
    #[inline(always)]
    fn add_assign(&mut self, rhs: Self) {
        *self = *self + rhs;
    }
}

impl Sub for ProcessDuration {
    type Output = Self;

    #[inline(always)]
    fn sub(self, rhs: Self) -> Self::Output {
        ProcessDuration {
            real: self.real - rhs.real,
            user: self.user - rhs.user,
            system: self.system - rhs.system,
        }
    }
}

impl SubAssign for ProcessDuration {
    #[inline(always)]
    fn sub_assign(&mut self, rhs: Self) {
        *self = *self - rhs;
    }
}
