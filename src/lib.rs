//! This crate allows you to measure how long it takes for a program to execute in different
//! clocks. It ports the functions of the [`boost-chrono`](https://boost.org/libs/chrono)
//! and [`boost-timer`](https://boost.org/libs/timer) libraries.
//!
//! The following clocks and their corresponding timers are implemented.
//!
//! * [`SystemClock`], [`SystemTimer`]
//! * [`SteadyClock`], [`SteadyTimer`] if supported by the system.
//! * [`HighResolutionClock`], [`HighResolutionTimer`]
//! * [`ProcessRealCPUClock`], [`ProcessRealCPUTimer`]
//! * [`ProcessUserCPUClock`], [`ProcessUserCPUTimer`]
//! * [`ProcessSystemCPUClock`], [`ProcessSystemCPUTimer`]
//! * [`ProcessCPUClock`], [`ProcessCPUTimer`]
//! * [`ThreadClock`], [`ThreadTimer`]
//!
//! See [`crate::clock`] to read more about their differences.
//!
//! # Usage
//!
//! Add this to your `Cargo.toml`:
//! ```toml
//! [dependencies]
//! howlong = "0.1"
//! ```
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

mod types;
pub use types::*;

pub mod clock;
pub use clock::*;

pub mod timer;
pub use timer::*;
