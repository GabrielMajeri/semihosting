/// Calls the host debugger through the semihosting ABI.
///
/// # Unsafety
///
/// This function is unsafe because using the wrong `op_num` / `param` could
/// cause undefined behavior (most likely crash the kernel).
#[inline]
pub unsafe fn call(op_num: u32, param: usize) -> usize {
    let ret;
    // This instruction is trapped by the debugger/supervisor with the help
    // of the CPU, which then processes the request.
    asm!("hlt $1" : "={x0}"(ret) : "i"(0xF000), "{w0}"(op_num), "{x1}"(param) :: "volatile");
    ret
}
