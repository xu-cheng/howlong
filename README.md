# howlong

[![Build Status](https://github.com/xu-cheng/howlong/workflows/build/badge.svg)](https://github.com/xu-cheng/howlong/actions)
[![Latest Version](https://img.shields.io/crates/v/howlong.svg)](https://crates.io/crates/howlong)
[![Rust Documentation](https://docs.rs/howlong/badge.svg)](https://docs.rs/howlong)

This crate allows you to measure how long it takes for a program to execute in different clocks. It ports the functions of the [`boost-chrono`](https://boost.org/libs/chrono) and [`boost-timer`](https://boost.org/libs/timer) libraries.

The following clocks and their corresponding timers are implemented.

* `SystemClock`, `SystemTimer`
* `SteadyClock`, `SteadyTimer` if supported by the system.
* `HighResolutionClock`, `HighResolutionTimer`
* `ProcessRealCPUClock`, `ProcessRealCPUTimer`
* `ProcessUserCPUClock`, `ProcessUserCPUTimer`
* `ProcessSystemCPUClock`, `ProcessSystemCPUTimer`
* `ProcessCPUClock`, `ProcessCPUTimer`
* `ThreadClock`, `ThreadTimer`

## Documentation

<https://docs.rs/howlong>

## Usage

Add this to your `Cargo.toml`:

```toml
[dependencies]
howlong = "0.1"
```

## Examples

```rust
let timer = howlong::HighResolutionTimer::new();
// do some computations
println!("{:?} have passed.", timer.elapsed());

let timer = howlong::ProcessCPUTimer::new();
// do other computations
println!("{}", timer.elapsed()); // 5.71s wall, 5.70s user + 0ns system = 5.70s CPU (99.8%)
```

## License

<sup>
Licensed under either of <a href="LICENSE-APACHE">Apache License, Version 2.0</a> or <a href="LICENSE-MIT">MIT license</a> at your option.
</sup>
<br>
<sub>
Unless you explicitly state otherwise, any contribution intentionally submitted for inclusion in this crate by you, as defined in the Apache-2.0 license, shall be dual licensed as above, without any additional terms or conditions.
</sub>
