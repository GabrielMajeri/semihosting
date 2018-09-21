use super::Op;
use core::mem;
use crate::error::*;

/// Returns info about the available memory of the running image.
pub fn heap_info() -> Result<HeapInfo> {
    let mut heap_info = unsafe { mem::zeroed() };
    let arg_ptr = &mut heap_info as *mut _;

    let ret = unsafe { Op::HeapInfo.call(arg_ptr as usize) };

    check(ret).map(|_| heap_info)
}

/// Structure describing the system stack and heap parameters.
#[derive(Debug, Copy, Clone)]
#[repr(C)]
pub struct HeapInfo {
    /// Base address of the heap.
    pub heap_base: usize,
    /// Size limit, in bytes, of the heap.
    pub heap_limit: usize,
    /// Base address of the stack.
    pub stack_base: usize,
    /// Size limit, in bytes, of the stack.
    pub stack_limit: usize,
}
