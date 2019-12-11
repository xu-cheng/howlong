// Ref: https://github.com/boostorg/chrono/tree/develop/include/boost/chrono/detail/inlined/mac

use crate::{Clock, Duration, Error, Result, TimePoint};
use core::mem;

#[allow(dead_code)]
#[path = "./posix.rs"]
mod posix;

mod mach {
    #![allow(clippy::all)]
    #![allow(dead_code)]
    #![allow(non_camel_case_types)]
    #![allow(non_snake_case)]
    #![allow(non_upper_case_globals)]
    #![allow(safe_packed_borrows)]
    include!(concat!(env!("OUT_DIR"), "/darwin_bindings.rs"));
}

/// A system clock.
// `gettimeofday` is the most precise "system time" available on macOS.
pub struct SystemClock;

impl Clock for SystemClock {
    type Output = TimePoint;

    fn try_now() -> Result<Self::Output> {
        let mut tv = libc::timeval {
            tv_sec: 0,
            tv_usec: 0,
        };
        let ret = unsafe { libc::gettimeofday(&mut tv, core::ptr::null_mut()) };
        if ret != 0 {
            return Err(Error::SystemError("gettimeofday", posix::errno()));
        }
        let d = Duration::from_secs(tv.tv_sec as u64) + Duration::from_micros(tv.tv_usec as u64);
        Ok(TimePoint(d))
    }
}

/// A steady clock.
// On macOS, it is based on `mach_absolute_time`.
// `mach_absolute_time() * MachInfo.numer / MachInfo.denom` is the number of
// nanoseconds since the computer booted up.
pub struct SteadyClock;

impl Clock for SteadyClock {
    type Output = TimePoint;

    fn try_now() -> Result<Self::Output> {
        let mut info: mach::mach_timebase_info_data_t = unsafe { mem::zeroed() };
        let ret = unsafe { mach::mach_timebase_info(&mut info) };
        if ret != 0 {
            return Err(Error::SystemError("mach_timebase_info", ret));
        }
        let absolute_time = unsafe { mach::mach_absolute_time() };
        let d = if info.numer == info.denom {
            Duration::from_nanos(absolute_time)
        } else {
            let factor = (info.numer as f64) / (info.denom as f64);
            Duration::from_nanos(absolute_time * (factor as u64))
        };
        Ok(TimePoint(d))
    }
}

pub use posix::{ProcessCPUClock, ProcessRealCPUClock, ProcessSystemCPUClock, ProcessUserCPUClock};

/// A clock to report the real thread wall-clock.
pub struct ThreadClock;

impl Clock for ThreadClock {
    type Output = TimePoint;

    fn try_now() -> Result<Self::Output> {
        let port = unsafe { mach::pthread_mach_thread_np(mach::pthread_self()) };
        let mut info: mach::thread_basic_info_data_t = unsafe { mem::zeroed() };
        let mut count: mach::mach_msg_type_number_t = mach::__THREAD_BASIC_INFO_COUNT;
        let ret = unsafe {
            mach::thread_info(
                port,
                mach::THREAD_BASIC_INFO,
                &mut info as *mut mach::thread_basic_info as *mut i32,
                &mut count,
            )
        };
        if ret != 0 {
            return Err(Error::SystemError("thread_info", ret));
        }
        let user = Duration::from_secs(info.user_time.seconds as u64)
            + Duration::from_micros(info.user_time.microseconds as u64);
        let system = Duration::from_secs(info.system_time.seconds as u64)
            + Duration::from_micros(info.system_time.microseconds as u64);
        Ok(TimePoint(user + system))
    }
}
