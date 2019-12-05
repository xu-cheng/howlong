use crate::{clock::*, Clock, Duration, ProcessDuration, ProcessTimePoint, TimePoint};
use std::marker::PhantomData;
use std::ops::Sub;
use std::rc::Rc;

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
    pub fn new() -> Self {
        Timer {
            running: true,
            start_time: <ClockType>::now(),
            _clock: PhantomData,
            _duration: PhantomData,
        }
    }

    pub fn is_running(&self) -> bool {
        self.running
    }

    pub fn is_stopped(&self) -> bool {
        !self.running
    }

    pub fn elapsed(&self) -> DurationType {
        if self.is_running() {
            <ClockType>::now() - self.start_time
        } else {
            self.start_time.into()
        }
    }

    pub fn start(&mut self) {
        if self.is_stopped() {
            self.running = true;
            self.start_time = <ClockType>::now();
        }
    }

    pub fn stop(&mut self) {
        if self.is_running() {
            self.running = false;
            self.start_time = <TimePointType>::from(<ClockType>::now() - self.start_time);
        }
    }

    pub fn resume(&mut self) {
        if self.is_stopped() {
            self.running = true;
            self.start_time = <TimePointType>::from(<ClockType>::now() - self.start_time);
        }
    }
}

pub type SystemTimer = Timer<SystemClock, TimePoint, Duration>;
#[cfg(have_steady_clock)]
pub type SteadyTimer = Timer<SteadyClock, TimePoint, Duration>;
pub type HighResolutionTimer = Timer<HighResolutionClock, TimePoint, Duration>;
pub type ProcessRealCPUTimer = Timer<ProcessRealCPUClock, TimePoint, Duration>;
pub type ProcessUserCPUTimer = Timer<ProcessUserCPUClock, TimePoint, Duration>;
pub type ProcessSystemCPUTimer = Timer<ProcessSystemCPUClock, TimePoint, Duration>;
pub type ProcessCPUTimer = Timer<ProcessCPUClock, ProcessTimePoint, ProcessDuration>;

pub struct ThreadTimer {
    inner: Timer<ThreadClock, TimePoint, Duration>,
    // makes type non-sync and non-send
    _no_sync: PhantomData<Rc<()>>,
}

impl ThreadTimer {
    pub fn new() -> Self {
        ThreadTimer {
            inner: Timer::<ThreadClock, TimePoint, Duration>::new(),
            _no_sync: PhantomData,
        }
    }

    #[inline(always)]
    pub fn is_running(&self) -> bool {
        self.inner.is_running()
    }

    #[inline(always)]
    pub fn is_stopped(&self) -> bool {
        self.inner.is_stopped()
    }

    #[inline(always)]
    pub fn elapsed(&self) -> Duration {
        self.inner.elapsed()
    }

    #[inline(always)]
    pub fn start(&mut self) {
        self.inner.start();
    }

    #[inline(always)]
    pub fn stop(&mut self) {
        self.inner.stop();
    }

    #[inline(always)]
    pub fn resume(&mut self) {
        self.inner.resume();
    }
}
