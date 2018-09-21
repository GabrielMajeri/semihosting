use super::Op;
use core::fmt;

/// Helper type for accessing the debugger's console.
///
/// This struct implements the [`core::fmt::Write`] trait.
///
/// Because the debugger requires NUL-terminated strings, the code has to allocate
/// a small (256 bytes) buffer on the stack and append the NUL byte.
/// It has to manually break up strings greater than that size.
pub struct DebugCon;

impl DebugCon {
    /// Reads a byte from the debugger's console.
    pub fn read(&self) -> u8 {
        let ret = unsafe { Op::ReadC.call(0) };
        ret as u8
    }
}

impl fmt::Write for DebugCon {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        // We could potentially end up in the middle of a UTF-8 character,
        // but that shouldn't be an issue since the char is pieced together
        // in the debugger's output.
        s.as_bytes().chunks(255).for_each(|ch| {
            // Handle chunks smaller than 255 bytes
            let ch_size = ch.len();

            let mut buf = [0; 256];
            buf[..ch_size].copy_from_slice(ch);
            buf[255] = 0;

            unsafe {
                Op::Write0.call(buf.as_ptr() as usize);
            }
        });

        Ok(())
    }
}
