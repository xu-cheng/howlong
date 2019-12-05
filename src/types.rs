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

impl From<Duration> for TimePoint {
    fn from(d: Duration) -> Self {
        TimePoint(d)
    }
}

impl Into<Duration> for TimePoint {
    fn into(self) -> Duration {
        self.0
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

impl From<ProcessDuration> for ProcessTimePoint {
    fn from(d: ProcessDuration) -> Self {
        ProcessTimePoint {
            real: d.real,
            user: d.user,
            system: d.system,
        }
    }
}

impl Into<ProcessDuration> for ProcessTimePoint {
    fn into(self) -> ProcessDuration {
        ProcessDuration {
            real: self.real,
            user: self.user,
            system: self.user,
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

impl ProcessDuration {
    /// Return the total CPU time. Equivalent to `user + system`.
    pub fn cpu_time(&self) -> Duration {
        self.user + self.system
    }

    /// Return the percentage of the CPU time that the process used.
    /// Equivalent to `(user + system) / real`.
    pub fn cpu_usage(&self) -> f64 {
        self.cpu_time().as_secs_f64() / self.real.as_secs_f64()
    }
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

impl core::fmt::Display for ProcessDuration {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(
            f,
            "{:?} wall, {:?} user + {:?} system = {:?} CPU ({:>3.1}%)",
            self.real,
            self.user,
            self.system,
            self.cpu_time(),
            self.cpu_usage() * 100f64,
        )
    }
}

/// A trait to represent a clock.
pub trait Clock {
    /// The returned timepoint type.
    type Output;

    /// Return the current timepoint.
    ///
    /// # Errors
    ///
    /// This function will return an error if acessing to the underlying system calls failed.
    fn try_now() -> Result<Self::Output>;

    /// Return the current timepoint.
    ///
    /// # Panics
    ///
    /// This function might panic when acessing to the underlying system calls failed.
    /// Use [`try_now`](Clock::try_now) if you want to handle the error.
    fn now() -> Self::Output {
        Self::try_now().expect("Failed to access the clock.")
    }
}
