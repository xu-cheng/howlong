//! Measure how long a program takes to execute.
//!
//! A varity of timers are implemented using different clocks.
//! See [`crate::clock`] to read more about their differences.
//!
//! # Examples
//!
//! ```
//! let timer = howlong::HighResolutionTimer::new();
//! // do some computations
//! println!("{:?} have passed.", timer.elapsed());
//!
//! let timer = howlong::ProcessCPUTimer::new();
//! // do other computations
//! println!("{}", timer.elapsed()); // 5.71s wall, 5.70s user + 0ns system = 5.70s CPU (99.8%)
//! ```

use crate::{clock::*, Clock, Duration, ProcessDuration, ProcessTimePoint, TimePoint};
use core::marker::PhantomData;
use core::ops::Sub;
use std::rc::Rc;

/// Generic timer.
pub struct Timer<ClockType, TimePointType, DurationType>
where
    ClockType: Clock<Output = TimePointType>,
    TimePointType: Copy + Sub<Output = DurationType> + From<DurationType> + Into<DurationType>,
{
    running: bool,
    start_time: TimePointType,
    _clock: PhantomData<ClockType>,
    _duration: PhantomData<DurationType>,
}

impl<ClockType, TimePointType, DurationType> Timer<ClockType, TimePointType, DurationType>
where
    ClockType: Clock<Output = TimePointType>,
    TimePointType: Copy + Sub<Output = DurationType> + From<DurationType> + Into<DurationType>,
{
    /// Construct a timer and start it.
    ///
    /// # Panics
    ///
    /// This function might panic when acessing to the underlying clock failed.
    #[allow(clippy::new_without_default)]
    pub fn new() -> Self {
        Timer {
            running: true,
            start_time: <ClockType>::now(),
            _clock: PhantomData,
            _duration: PhantomData,
        }
    }

    /// Return true if the timer is running.
    pub fn is_running(&self) -> bool {
        self.running
    }

    /// Return true if the timer is not running.
    pub fn is_stopped(&self) -> bool {
        !self.running
    }

    /// Return the accumulated elapsed times as of the previous [`stop()`](#method.stop)
    /// if the timer is stopped. Otherwise, the elapsed times accumulated between the most
    /// recent call to [`start()`](#method.start) or [`resume()`](#method.resume) and the
    /// current time values.
    ///
    /// # Panics
    ///
    /// This function might panic when acessing to the underlying clock failed.
    pub fn elapsed(&self) -> DurationType {
        if self.is_running() {
            <ClockType>::now() - self.start_time
        } else {
            self.start_time.into()
        }
    }

    /// If the timer is not running, reset and start the timer.
    ///
    /// # Panics
    ///
    /// This function might panic when acessing to the underlying clock failed.
    pub fn start(&mut self) {
        if self.is_stopped() {
            self.running = true;
            self.start_time = <ClockType>::now();
        }
    }

    /// Stop the timer.
    ///
    /// # Panics
    ///
    /// This function might panic when acessing to the underlying clock failed.
    pub fn stop(&mut self) {
        if self.is_running() {
            self.running = false;
            self.start_time = <TimePointType>::from(<ClockType>::now() - self.start_time);
        }
    }

    /// Resume the timer, accumulating additional elapsed time.
    ///
    /// # Panics
    ///
    /// This function might panic when acessing to the underlying clock failed.
    pub fn resume(&mut self) {
        if self.is_stopped() {
            self.running = true;
            self.start_time = <TimePointType>::from(<ClockType>::now() - self.start_time);
        }
    }
}

/// A timer to measure system time.
pub type SystemTimer = Timer<SystemClock, TimePoint, Duration>;

#[cfg(have_steady_clock)]
#[doc = "A timer using steady clock."]
pub type SteadyTimer = Timer<SteadyClock, TimePoint, Duration>;

/// A timer using high resolution clock.
pub type HighResolutionTimer = Timer<HighResolutionClock, TimePoint, Duration>;

/// A timer to measure the real process wall-clock.
pub type ProcessRealCPUTimer = Timer<ProcessRealCPUClock, TimePoint, Duration>;

/// A timer to measure the user cpu-clock.
pub type ProcessUserCPUTimer = Timer<ProcessUserCPUClock, TimePoint, Duration>;

/// A timer to measure the system cpu-clock.
pub type ProcessSystemCPUTimer = Timer<ProcessSystemCPUClock, TimePoint, Duration>;

/// A timer to measure real, user-CPU, and system-CPU clocks at the same time.
pub type ProcessCPUTimer = Timer<ProcessCPUClock, ProcessTimePoint, ProcessDuration>;

/// A timer to measure thread CPU time.
pub struct ThreadTimer {
    inner: Timer<ThreadClock, TimePoint, Duration>,
    // makes type non-sync and non-send
    _no_sync: PhantomData<Rc<()>>,
}

impl ThreadTimer {
    /// Construct a timer and start it.
    ///
    /// # Panics
    ///
    /// This function might panic when acessing to the underlying clock failed.
    #[allow(clippy::new_without_default)]
    pub fn new() -> Self {
        ThreadTimer {
            inner: Timer::<ThreadClock, TimePoint, Duration>::new(),
            _no_sync: PhantomData,
        }
    }

    /// Return true if the timer is running.
    #[inline(always)]
    pub fn is_running(&self) -> bool {
        self.inner.is_running()
    }

    /// Return true if the timer is not running.
    #[inline(always)]
    pub fn is_stopped(&self) -> bool {
        self.inner.is_stopped()
    }

    /// Return the accumulated elapsed times as of the previous [`stop()`](#method.stop)
    /// if the timer is stopped. Otherwise, the elapsed times accumulated between the most
    /// recent call to [`start()`](#method.start) or [`resume()`](#method.resume) and the
    /// current time values.
    ///
    /// # Panics
    ///
    /// This function might panic when acessing to the underlying clock failed.
    #[inline(always)]
    pub fn elapsed(&self) -> Duration {
        self.inner.elapsed()
    }

    /// If the timer is not running, reset and start the timer.
    ///
    /// # Panics
    ///
    /// This function might panic when acessing to the underlying clock failed.
    #[inline(always)]
    pub fn start(&mut self) {
        self.inner.start();
    }

    /// Stop the timer.
    ///
    /// # Panics
    ///
    /// This function might panic when acessing to the underlying clock failed.
    #[inline(always)]
    pub fn stop(&mut self) {
        self.inner.stop();
    }

    /// Resume the timer, accumulating additional elapsed time.
    ///
    /// # Panics
    ///
    /// This function might panic when acessing to the underlying clock failed.
    #[inline(always)]
    pub fn resume(&mut self) {
        self.inner.resume();
    }
}
