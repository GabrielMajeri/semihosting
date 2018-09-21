use super::Op;
use core::{slice, str};
use crate::error::*;

/// Retrieves the arguments the semihosted binary has been called with.
///
/// The only parameter is a reference to a buffer which should be big enough to
/// store the command line arguments. Note that the minimum length required to
/// be supported is 80 bytes.
///
/// This function will panic if the command line contains invalid UTF-8 characters.
pub fn cmd_line(buffer: &mut [u8]) -> Result<&str> {
    let mut arg_block = [buffer.as_mut_ptr() as usize, buffer.len()];

    let ret = unsafe { Op::GetCmdLine.call(arg_block.as_mut_ptr() as usize) };

    check(ret).map(|_| {
        let slice = unsafe {
            let data = arg_block[0] as *const u8;
            let len = arg_block[1];
            slice::from_raw_parts(data, len)
        };

        str::from_utf8(&slice).expect("Command line contains invalid UTF-8")
    })
}
