use super::Op;
use crate::error::*;

/// Returns the number of hundreths of a second since execution started.
///
/// Due to the overhead of the call, this function should _not_ be used for benchmarking.
pub fn clock() -> Result<usize> {
    let ret = unsafe { Op::Clock.call(0) };
    check(ret)
}

/// Returns the UNIX Epoch time, in seconds.
pub fn time() -> Result<usize> {
    // No need to `check` result, docs say it always succeeds.
    let ret = unsafe { Op::Time.call(0) };
    check(ret)
}

/// Returns the number of ticks since execution started.
///
/// Use [`tick_freq`] to determine the tick frequency.
///
/// Some implementations might not support this function,
/// in which case it will always return `Err`.
pub fn elapsed() -> Result<u64> {
    let arg_block = [0u64];

    let ret = unsafe { Op::Elapsed.call(arg_block.as_ptr() as usize) };

    check(ret).map(|_| arg_block[0])
}

/// Returns the tick frequency.
///
/// Some implementations might not support this function,
/// in which case it will always return `Err`.
pub fn tick_freq() -> Result<usize> {
    let ret = unsafe { Op::TickFreq.call(0) };
    check(ret)
}
