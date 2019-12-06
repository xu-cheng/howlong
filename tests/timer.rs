extern crate criterion;
extern crate howlong;

use criterion::black_box;
use howlong::{timer::*, Duration};
use std::thread;

macro_rules! test_timer {
    ($name: ident, $timer: ty) => {
        #[test]
        fn $name() {
            let ten_millis = Duration::from_millis(10);
            let twenty_millis = Duration::from_millis(20);
            let mut timer = <$timer>::new();
            assert!(timer.is_running());
            thread::sleep(ten_millis);
            let elapsed = timer.elapsed();
            assert!(timer.is_running());
            assert!(elapsed >= ten_millis);
            timer.stop();
            assert!(timer.is_stopped());
            thread::sleep(ten_millis);
            assert!(timer.elapsed() >= elapsed);
            timer.resume();
            thread::sleep(ten_millis);
            timer.stop();
            assert!(timer.elapsed() >= twenty_millis);
            timer.start();
            timer.stop();
            assert!(timer.elapsed() < twenty_millis);
        }
    };
}

test_timer!(test_system_timer, SystemTimer);
#[cfg(have_steady_timer)]
test_timer!(test_steady_timer, SteadyTimer);
test_timer!(test_high_resolution_timer, HighResolutionTimer);
test_timer!(test_process_real_cpu_timer, ProcessRealCPUTimer);

fn computation_task() {
    black_box((0..1_000_000).fold(0, |old, new| black_box(old ^ new)));
}

#[test]
fn test_process_user_cpu_timer() {
    let timer = ProcessUserCPUTimer::new();
    computation_task();
    let elapsed = timer.elapsed();
    assert!(elapsed > Duration::from_nanos(0));
}

#[test]
fn test_process_system_cpu_timer() {
    let timer = ProcessSystemCPUTimer::new();
    let elapsed = timer.elapsed();
    assert!(elapsed < Duration::from_nanos(10));
}

#[test]
fn test_process_cpu_timer() {
    let timer = ProcessCPUTimer::new();
    (0..4)
        .map(|_| thread::spawn(|| computation_task()))
        .for_each(|t| {
            let _ = t.join();
        });
    let elapsed = timer.elapsed();
    println!("{}", elapsed);
    assert!(elapsed.cpu_usage() >= 1f64);
    assert!(elapsed.real > Duration::from_nanos(0));
    assert!(elapsed.user > Duration::from_nanos(0));
    assert!(elapsed.user + elapsed.system >= elapsed.real);
}

#[test]
fn test_thread_timer() {
    let timer_outer = ThreadTimer::new();
    let elapsed_inner = thread::spawn(|| {
        let timer_inner = ThreadTimer::new();
        computation_task();
        timer_inner.elapsed()
    })
    .join()
    .unwrap();
    let elapsed_outter = timer_outer.elapsed();
    assert!(elapsed_inner > Duration::from_nanos(0));
    assert!(elapsed_inner > elapsed_outter);
}
