use super::Op;

/// Reports to the debugger that the application has exited, either at the end of
/// normal execution or due to an exception.
///
/// The first parameter is the reason for stopping execution. The second
/// parameter is an exit code, which could have a specific meaning depending
/// on the exit reason.
///
/// It is in theory possible to continue execution after this call, if a debugger
/// used the Remote Debug Interface to request it. This behavior is not supported.
pub fn exit(reason: ExitReason, code: u32) -> ! {
    let exception_ty = 0x2_00_00 | reason as usize;
    let code = code as usize;

    let mut arg_block = [exception_ty, code];

    unsafe {
        Op::Exit.call(arg_block.as_mut_ptr() as usize);
    }

    unreachable!("Execution continued after `exit` call")
}

/// The sole argument passed to an `exit` call, describing the reason for the
/// termination of execution.
///
/// Most of these correspond to exceptions, and they are intended to be called
/// in exception handlers to notify the attached debugger. The exception is the
/// `ApplicationExit` variant, which is intended to notify the host that the
/// application code finished executing, and returned an exit code.
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
// Variant names are pretty self-explanatory
#[allow(missing_docs)]
pub enum ExitReason {
    BranchThroughZero = 0x0,
    UndefinedInstr = 0x1,
    SoftwareInterrupt = 0x2,
    PrefetchAbort = 0x3,
    DataAbort = 0x4,
    AddressException = 0x5,
    IRQ = 0x6,
    FIQ = 0x7,
    BreakPoint = 0x20,
    WatchPoint = 0x21,
    StepComplete = 0x22,
    RunTimeErrorUnknown = 0x23,
    InternalError = 0x24,
    UserInterruption = 0x25,
    ApplicationExit = 0x26,
    StackOverflow = 0x27,
    DivisionByZero = 0x28,
    OSSpecific = 0x29,
}
