# howlong

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

<https://doc.rs/howlong>

## Usage

Add this to your `Cargo.toml`:

```toml
[dependencies]
howlong = "0.1"
```

## Examples

```rust
use howlong::*;

let timer = HighResolutionTimer::new();
// do some computations
println!("{:?} have passed.", timer.elapsed());

let timer = ProcessCPUTimer::new();
// do other computations
println!("{}", timer.elapsed());
```

## License

<sup>
Licensed under either of <a href="LICENSE-APACHE">Apache License, Version 2.0</a> or <a href="LICENSE-MIT">MIT license</a> at your option.
</sup>
<br>
<sub>
Unless you explicitly state otherwise, any contribution intentionally submitted for inclusion in this crate by you, as defined in the Apache-2.0 license, shall be dual licensed as above, without any additional terms or conditions.
</sub>
