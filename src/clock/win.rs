// Ref: https://github.com/boostorg/chrono/tree/develop/include/boost/chrono/detail/inlined/win

extern crate winapi;

use winapi::shared::minwindef::FILETIME;
use winapi::um::{
    errhandlingapi::GetLastError,
    processthreadsapi::{GetCurrentProcess, GetCurrentThread, GetProcessTimes, GetThreadTimes},
    profileapi::{QueryPerformanceCounter, QueryPerformanceFrequency},
    sysinfoapi::GetSystemTimeAsFileTime,
    winnt::LARGE_INTEGER,
};

use crate::{Duration, Error, ProcessTimePoint, Result, TimePoint};

/// A system clock.
pub struct SystemClock;

impl SystemClock {
    pub fn now() -> Result<TimePoint> {
        unimplemented!();
    }
}

/// A steady clock.
pub struct SteadyClock;

impl SteadyClock {
    pub fn now() -> Result<TimePoint> {
        unimplemented!();
    }
}

/// A clock to report the real process wall-clock.
pub struct ProcessRealCPUClock;

impl ProcessRealCPUClock {
    pub fn now() -> Result<TimePoint> {
        unimplemented!();
    }
}

/// A clock to report the user cpu-clock.
pub struct ProcessUserCPUClock;

impl ProcessUserCPUClock {
    pub fn now() -> Result<TimePoint> {
        unimplemented!();
    }
}

/// A clock to report the system cpu-clock.
pub struct ProcessSystemCPUClock;

impl ProcessSystemCPUClock {
    pub fn now() -> Result<TimePoint> {
        unimplemented!();
    }
}

/// A clock to report real, user-CPU, and system-CPU clocks.
pub struct ProcessCPUClock;

impl ProcessCPUClock {
    pub fn now() -> Result<ProcessTimePoint> {
        unimplemented!();
    }
}

/// A clock to report the real thread wall-clock.
pub struct ThreadClock;

impl ThreadClock {
    pub fn now() -> Result<TimePoint> {
        unimplemented!();
    }
}
