// Ref: https://github.com/boostorg/chrono/tree/develop/include/boost/chrono/detail/inlined/posix

extern crate errno;
extern crate libc;

use crate::{Clock, Duration, Error, ProcessTimePoint, Result, TimePoint};

pub(crate) fn errno() -> i32 {
    errno::errno().into()
}

/// A system clock.
pub struct SystemClock;

impl Clock for SystemClock {
    type Output = TimePoint;

    fn try_now() -> Result<Self::Output> {
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

#[cfg(have_steady_clock)]
#[doc = "A steady clock."]
pub struct SteadyClock;

#[cfg(have_steady_clock)]
impl Clock for SteadyClock {
    type Output = TimePoint;

    fn try_now() -> Result<Self::Output> {
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

impl Clock for ProcessRealCPUClock {
    type Output = TimePoint;

    fn try_now() -> Result<Self::Output> {
        let (c, _) = times()?;
        let factor = tick_factor()?;
        let d = Duration::from_nanos((c as u64) * factor);
        Ok(TimePoint(d))
    }
}

/// A clock to report the user cpu-clock.
pub struct ProcessUserCPUClock;

impl Clock for ProcessUserCPUClock {
    type Output = TimePoint;

    fn try_now() -> Result<Self::Output> {
        let (_, tm) = times()?;
        let factor = tick_factor()?;
        let d = Duration::from_nanos(((tm.tms_utime + tm.tms_cutime) as u64) * factor);
        Ok(TimePoint(d))
    }
}

/// A clock to report the system cpu-clock.
pub struct ProcessSystemCPUClock;

impl Clock for ProcessSystemCPUClock {
    type Output = TimePoint;

    fn try_now() -> Result<Self::Output> {
        let (_, tm) = times()?;
        let factor = tick_factor()?;
        let d = Duration::from_nanos(((tm.tms_stime + tm.tms_cstime) as u64) * factor);
        Ok(TimePoint(d))
    }
}

/// A clock to report real, user-CPU, and system-CPU clocks.
pub struct ProcessCPUClock;

impl Clock for ProcessCPUClock {
    type Output = ProcessTimePoint;

    fn try_now() -> Result<Self::Output> {
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

impl Clock for ThreadClock {
    type Output = TimePoint;

    fn try_now() -> Result<Self::Output> {
        let mut ts = libc::timespec {
            tv_sec: 0,
            tv_nsec: 0,
        };
        let clock_id = get_thread_clock_id()?;
        let ret = unsafe { libc::clock_gettime(clock_id, &mut ts) };
        if ret != 0 {
            return Err(Error::SystemError("clock_gettime", errno()));
        }
        let d = Duration::from_secs(ts.tv_sec as u64) + Duration::from_nanos(ts.tv_nsec as u64);
        Ok(TimePoint(d))
    }
}
