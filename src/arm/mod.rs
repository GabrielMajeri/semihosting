//! Semihosted calls defined by ARM.

/// Semihosted system calls defined by ARM.
/// All the system call op codes below `0xFF` are reserved.
#[repr(u32)]
enum Op {
    Open = 0x1,
    Close = 0x2,
    Write0 = 0x4,
    Write = 0x5,
    Read = 0x6,
    ReadC = 0x7,
    IsTTY = 0x9,
    Seek = 0xA,
    FLen = 0xC,
    TmpName = 0xD,
    Remove = 0xE,
    Rename = 0xF,
    Clock = 0x10,
    Time = 0x11,
    System = 0x12,
    Errno = 0x13,
    GetCmdLine = 0x15,
    HeapInfo = 0x16,
    Exit = 0x18,
    Elapsed = 0x30,
    TickFreq = 0x31,
}

impl Op {
    /// Calls this ARM-defined call with a certain parameter.
    #[inline]
    unsafe fn call(self, param: usize) -> usize {
        crate::call(self as u32, param)
    }
}

mod cmd;
pub use self::cmd::*;

mod dbg;
pub use self::dbg::*;

mod exit;
pub use self::exit::*;

mod exts;
pub use self::exts::*;

mod fs;
pub use self::fs::*;

mod heap;
pub use self::heap::*;

mod sys;
pub use self::sys::*;

mod time;
pub use self::time::*;
