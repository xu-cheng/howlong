// Ref: https://github.com/boostorg/chrono/tree/develop/include/boost/chrono/detail/inlined/win

extern crate winapi;

use core::mem;
use winapi::shared::minwindef::FILETIME;
use winapi::um::{
    errhandlingapi::GetLastError,
    processthreadsapi::{GetCurrentProcess, GetCurrentThread, GetProcessTimes, GetThreadTimes},
    profileapi::{QueryPerformanceCounter, QueryPerformanceFrequency},
    sysinfoapi::GetSystemTimeAsFileTime,
    winnt::LARGE_INTEGER,
};

use crate::{Clock, Duration, Error, ProcessTimePoint, Result, TimePoint};

fn errno() -> i32 {
    unsafe { GetLastError() as i32 }
}

#[inline(always)]
fn filetime_to_duration(ft: FILETIME) -> Duration {
    Duration::from_nanos((((ft.dwHighDateTime as u64) << 32) | (ft.dwLowDateTime as u64)) * 100)
}

/// A system clock.
pub struct SystemClock;

impl Clock for SystemClock {
    type Output = TimePoint;

    fn try_now() -> Result<Self::Output> {
        let mut ft = FILETIME {
            dwLowDateTime: 0,
            dwHighDateTime: 0,
        };
        unsafe { GetSystemTimeAsFileTime(&mut ft) };
        Ok(TimePoint(filetime_to_duration(ft)))
    }
}

/// A steady clock.
pub struct SteadyClock;

impl Clock for SteadyClock {
    type Output = TimePoint;

    fn try_now() -> Result<Self::Output> {
        let mut freq: LARGE_INTEGER = unsafe { mem::zeroed() };
        let ret = unsafe { QueryPerformanceFrequency(&mut freq) };
        if ret == 0 {
            return Err(Error::SystemError("QueryPerformanceFrequency", errno()));
        }
        let factor = (1_000_000_000 / unsafe { *freq.QuadPart() }) as u64;
        let mut cnt: LARGE_INTEGER = unsafe { mem::zeroed() };
        let ret = unsafe { QueryPerformanceCounter(&mut cnt) };
        if ret == 0 {
            return Err(Error::SystemError("QueryPerformanceCounter", errno()));
        }
        let d = Duration::from_nanos(factor * unsafe { *cnt.QuadPart() as u64 });
        Ok(TimePoint(d))
    }
}

/// A clock to report the real process wall-clock.
pub struct ProcessRealCPUClock;

impl Clock for ProcessRealCPUClock {
    type Output = TimePoint;

    fn try_now() -> Result<Self::Output> {
        SteadyClock::try_now()
    }
}

#[inline(always)]
fn get_process_times() -> Result<(FILETIME, FILETIME)> {
    let mut creation = FILETIME {
        dwLowDateTime: 0,
        dwHighDateTime: 0,
    };
    let mut exit = FILETIME {
        dwLowDateTime: 0,
        dwHighDateTime: 0,
    };
    let mut user_time = FILETIME {
        dwLowDateTime: 0,
        dwHighDateTime: 0,
    };
    let mut system_time = FILETIME {
        dwLowDateTime: 0,
        dwHighDateTime: 0,
    };
    let ret = unsafe {
        GetProcessTimes(
            GetCurrentProcess(),
            &mut creation,
            &mut exit,
            &mut system_time,
            &mut user_time,
        )
    };
    if ret == 0 {
        return Err(Error::SystemError("GetProcessTimes", errno()));
    }
    Ok((user_time, system_time))
}

/// A clock to report the user cpu-clock.
pub struct ProcessUserCPUClock;

impl Clock for ProcessUserCPUClock {
    type Output = TimePoint;

    fn try_now() -> Result<Self::Output> {
        let (user_time, _) = get_process_times()?;
        Ok(TimePoint(filetime_to_duration(user_time)))
    }
}

/// A clock to report the system cpu-clock.
pub struct ProcessSystemCPUClock;

impl Clock for ProcessSystemCPUClock {
    type Output = TimePoint;

    fn try_now() -> Result<Self::Output> {
        let (_, system_time) = get_process_times()?;
        Ok(TimePoint(filetime_to_duration(system_time)))
    }
}

/// A clock to report real, user-CPU, and system-CPU clocks.
pub struct ProcessCPUClock;

impl Clock for ProcessCPUClock {
    type Output = ProcessTimePoint;

    fn try_now() -> Result<Self::Output> {
        let (user_time, system_time) = get_process_times()?;
        Ok(ProcessTimePoint {
            real: SteadyClock::try_now()?.0,
            user: filetime_to_duration(user_time),
            system: filetime_to_duration(system_time),
        })
    }
}

/// A clock to report the real thread wall-clock.
pub struct ThreadClock;

impl Clock for ThreadClock {
    type Output = TimePoint;

    fn try_now() -> Result<Self::Output> {
        let mut creation = FILETIME {
            dwLowDateTime: 0,
            dwHighDateTime: 0,
        };
        let mut exit = FILETIME {
            dwLowDateTime: 0,
            dwHighDateTime: 0,
        };
        let mut user_time = FILETIME {
            dwLowDateTime: 0,
            dwHighDateTime: 0,
        };
        let mut system_time = FILETIME {
            dwLowDateTime: 0,
            dwHighDateTime: 0,
        };
        let ret = unsafe {
            GetThreadTimes(
                GetCurrentThread(),
                &mut creation,
                &mut exit,
                &mut system_time,
                &mut user_time,
            )
        };
        if ret == 0 {
            return Err(Error::SystemError("GetThreadTimes", errno()));
        }
        let user = filetime_to_duration(user_time);
        let system = filetime_to_duration(system_time);
        Ok(TimePoint(user + system))
    }
}
