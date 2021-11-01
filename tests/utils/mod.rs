#[inline(never)]
pub(crate) fn black_box<T>(input: T) -> T {
    unsafe {
        let ret = core::ptr::read_volatile(&input);
        core::mem::forget(input);
        ret
    }
}

pub(crate) fn computation_task() -> usize {
    fn fib(n: usize) -> usize {
        let mut i = 0;
        let mut sum = 0;
        let mut last = 0;
        let mut curr = 1usize;
        while i < n - 1 {
            sum = curr.wrapping_add(last);
            last = curr;
            curr = sum;
            i += 1;
        }
        sum
    }
    fib(100_000_000) % 1000
}

pub(crate) fn multithreading_task() -> usize {
    (0..4)
        .map(|_| std::thread::spawn(computation_task))
        .map(|t| t.join().unwrap())
        .sum()
}
