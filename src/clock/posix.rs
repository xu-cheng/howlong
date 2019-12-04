// Ref: https://github.com/boostorg/chrono/tree/develop/include/boost/chrono/detail/inlined/posix

extern crate errno;
extern crate libc;

use crate::{Duration, Error, ProcessTimePoint, Result, TimePoint};

pub(crate) fn errno() -> i32 {
    errno::errno().into()
}

/// A system clock.
pub struct SystemClock;

impl SystemClock {
    pub fn now() -> Result<TimePoint> {
        let mut ts = libc::timespec {
            tv_sec: 0,
            tv_nsec: 0,
        };
        let ret = unsafe { libc::clock_gettime(libc::CLOCK_REALTIME, &mut ts) };
        if ret != 0 {
            return Err(Error::SystemError("clock_gettime", errno()));
        }
        let d = Duration::from_secs(ts.tv_sec as u64) + Duration::from_nanos(ts.tv_nsec as u64);
        Ok(TimePoint(d))
    }
}

/// A steady clock.
#[cfg(have_steady_clock)]
pub struct SteadyClock;

#[cfg(have_steady_clock)]
impl SteadyClock {
    pub fn now() -> Result<TimePoint> {
        let mut ts = libc::timespec {
            tv_sec: 0,
            tv_nsec: 0,
        };
        let ret = unsafe { libc::clock_gettime(libc::CLOCK_MONOTONIC, &mut ts) };
        if ret != 0 {
            return Err(Error::SystemError("clock_gettime", errno()));
        }
        let d = Duration::from_secs(ts.tv_sec as u64) + Duration::from_nanos(ts.tv_nsec as u64);
        Ok(TimePoint(d))
    }
}

fn tick_factor() -> Result<u64> {
    let factor = unsafe { libc::sysconf(libc::_SC_CLK_TCK) };
    if factor <= 0 {
        return Err(Error::SystemError("sysconf(_SC_CLK_TCK)", errno()));
    }
    if factor > 1_000_000_000 {
        return Err(Error::ClkFreqTooHigh);
    }
    Ok((1_000_000_000 / factor) as u64)
}

#[inline(always)]
fn times() -> Result<(libc::clock_t, libc::tms)> {
    let mut tm = libc::tms {
        tms_utime: 0,
        tms_stime: 0,
        tms_cutime: 0,
        tms_cstime: 0,
    };
    let ret = unsafe { libc::times(&mut tm) };
    if ret == -1i64 as libc::clock_t {
        return Err(Error::SystemError("times", errno()));
    }
    Ok((ret, tm))
}

/// A clock to report the real process wall-clock.
pub struct ProcessRealCPUClock;

impl ProcessRealCPUClock {
    pub fn now() -> Result<TimePoint> {
        let (c, _) = times()?;
        let factor = tick_factor()?;
        let d = Duration::from_nanos((c as u64) * factor);
        Ok(TimePoint(d))
    }
}

/// A clock to report the user cpu-clock.
pub struct ProcessUserCPUClock;

impl ProcessUserCPUClock {
    pub fn now() -> Result<TimePoint> {
        let (_, tm) = times()?;
        let factor = tick_factor()?;
        let d = Duration::from_nanos(((tm.tms_utime + tm.tms_cutime) as u64) * factor);
        Ok(TimePoint(d))
    }
}

/// A clock to report the system cpu-clock.
pub struct ProcessSystemCPUClock;

impl ProcessSystemCPUClock {
    pub fn now() -> Result<TimePoint> {
        let (_, tm) = times()?;
        let factor = tick_factor()?;
        let d = Duration::from_nanos(((tm.tms_stime + tm.tms_cstime) as u64) * factor);
        Ok(TimePoint(d))
    }
}

/// A clock to report real, user-CPU, and system-CPU clocks.
pub struct ProcessCPUClock;

impl ProcessCPUClock {
    pub fn now() -> Result<ProcessTimePoint> {
        let (c, tm) = times()?;
        let factor = tick_factor()?;
        Ok(ProcessTimePoint {
            real: Duration::from_nanos((c as u64) * factor),
            user: Duration::from_nanos(((tm.tms_utime + tm.tms_cutime) as u64) * factor),
            system: Duration::from_nanos(((tm.tms_stime + tm.tms_cstime) as u64) * factor),
        })
    }
}

/// A clock to report the real thread wall-clock.
pub struct ThreadClock;

#[cfg(not(have_clock_thread_cputime_id))]
extern "C" {
    fn pthread_getcpuclockid(
        thread_id: libc::pthread_t,
        clock_id: *mut libc::clockid_t,
    ) -> libc::c_int;
}

impl ThreadClock {
    #[cfg(have_clock_thread_cputime_id)]
    #[inline(always)]
    fn get_thread_clock_id() -> Result<libc::clockid_t> {
        Ok(libc::CLOCK_THREAD_CPUTIME_ID)
    }

    #[cfg(not(have_clock_thread_cputime_id))]
    #[inline(always)]
    fn get_thread_clock_id() -> Result<libc::clockid_t> {
        let mut clock_id: libc::clockid_t = 0;
        let ret = unsafe { pthread_getcpuclockid(libc::pthread_self(), &mut clock_id) };
        if ret != 0 {
            return Err(Error::SystemError("pthread_getcpuclockid", errno()));
        }
        Ok(clock_id)
    }

    pub fn now() -> Result<TimePoint> {
        let mut ts = libc::timespec {
            tv_sec: 0,
            tv_nsec: 0,
        };
        let clock_id = Self::get_thread_clock_id()?;
        let ret = unsafe { libc::clock_gettime(clock_id, &mut ts) };
        if ret != 0 {
            return Err(Error::SystemError("clock_gettime", errno()));
        }
        let d = Duration::from_secs(ts.tv_sec as u64) + Duration::from_nanos(ts.tv_nsec as u64);
        Ok(TimePoint(d))
    }
}
