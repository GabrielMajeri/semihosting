use super::Op;

/// Returns the value of the `errno` variable of the host.
pub fn errno() -> i32 {
    let ret = unsafe { Op::Errno.call(0) };

    ret as i32
}

/// Executes a command in the host's shell and returns the exit code.
///
/// The command line string must be NUL-terminated.
pub fn system(cmd: &str) -> i32 {
    assert_eq!(
        cmd.bytes().last().unwrap(),
        0,
        "String must be NUL terminated"
    );

    let arg_block = [cmd.as_ptr() as usize, cmd.len()];

    let ret = unsafe { Op::System.call(arg_block.as_ptr() as usize) };

    ret as i32
}
