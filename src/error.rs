use core::result;

/// Result type returned by most functions in this crate.
///
/// Use the [`crate::arm::errno`] function to retrieve more information
/// about an error.
pub type Result<T> = result::Result<T, ()>;

/// Checks the return code of a system call,
/// assuming the value `-1` indicates a failed call.
pub fn check(ret: usize) -> Result<usize> {
    if ret as isize != -1 {
        Ok(ret)
    } else {
        Err(())
    }
}
