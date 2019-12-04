//! Measure the current time using different clocks.
//!
//! The following list of clocks are implemented.
//! * [`SystemClock`]: It provides access to the current wall-clock time from the system-wide
//!   real-time clock.
//! * [`SteadyClock`]: It provides access to system-wide steady clock. There is no fixed
//!   relationship between values returned by `SteadyClock::now()` and wall-clock time
//! * [`HighResolutionClock`]: Default to [`SteadyClock`] if available, otherwise fallback to
//!   [`SystemClock`].
//! * [`ProcessRealCPUClock`]: It provides access to the real process wall-clock steady clock,
//!   i.e. the real CPU-time clock of the calling process.
//! * [`ProcessUserCPUClock`]: It provides access to the user CPU-time steady clock of the
//!   calling process.
//! * [`ProcessSystemCPUClock`]: It provides access to the system CPU-time steady clock of
//!   the calling process.
//! * [`ProcessCPUClock`]: It provides access to real, user-CPU, and system-CPU clocks at
//!   the same time.
//! * [`ThreadClock`]: It provides access to the real thread wall-clock, i.e. the real CPU-time
//!   clock of the calling thread.
//!
//! # Implementations
//!
//! The Implementations of the clocks are based on
//! [`boost-chrono` library](http://boost.org/libs/chrono)
//! The following table listes the underlying APIs for different clocks.
//!
//! | Clock | Posix | Darwin | Windows |
//! |-------|-------|--------|---------|
//! | [`SystemClock`] | `clock_gettime(CLOCK_REALTIME)` | `gettimeofday` | `GetSystemTimeAsFileTime` |
//! | [`SteadyClock`] | `clock_gettime(CLOCK_MONOTONIC)` | `mach_timebase_info`, `mach_absolute_time` | `QueryPerformanceCounter`, `QueryPerformanceFrequency` |
//! | [`ProcessRealCPUClock`] | `times` | `times` | `QueryPerformanceCounter`, `QueryPerformanceFrequency` |
//! | [`ProcessUserCPUClock`] | `times` | `times` | `GetProcessTimes` |
//! | [`ProcessSystemCPUClock`] | `times` | `times` | `GetProcessTimes` |
//! | [`ProcessCPUClock`] | `times` | `times` | `GetProcessTimes`, `QueryPerformanceCounter`, `QueryPerformanceFrequency` |
//! | [`ThreadClock`] | `clock_gettime(pthread_getcpuclockid)` | `thread_info` | `GetThreadTimes` |

cfg_if::cfg_if! {
    if #[cfg(any(target_os = "macos", target_os = "ios"))] {
        // Darwin
        mod darwin;
        pub use darwin::*;
    } else if #[cfg(windows)] {
        // Windows
        mod win;
        pub use win::*;
    } else if #[cfg(unix)] {
        // Posix
        mod posix;
        pub use posix::*;
    } else {
        compile_error!("unsupported platform.");
    }
}

cfg_if::cfg_if! {
    if #[cfg(have_steady_clock)] {
        /// A high resolution clock.
        pub type HighResolutionClock = SteadyClock;
    } else {
        /// A high resolution clock.
        pub type HighResolutionClock = SystemClock;
    }
}
