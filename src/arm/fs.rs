use super::Op;
use core::{fmt, ptr, result};
use crate::error::*;

/// Handle to a file on the host system.
///
/// The file is closed when dropped.
///
/// You can use the associated functions to operate on this file. This structure
/// also implements the [`core::fmt::Write`] trait.
pub struct File(ptr::NonNull<()>);

impl File {
    /// Opens a file on the host system.
    ///
    /// The given path can be relative to the host's current working directory
    /// or absolute. The path must be NUL-terminated.
    ///
    pub fn open(path: &str, mode: OpenMode) -> Result<File> {
        assert_eq!(
            path.bytes().last().unwrap(),
            0,
            "Path is missing the NUL-terminator"
        );

        let arg_block = [path.as_ptr() as usize, mode as usize, path.len() - 1];

        let ret = unsafe { Op::Open.call(arg_block.as_ptr() as usize) };

        let handle = check(ret)?;

        // The docs specify that a successful call returns a "nonzero handle"
        let handle = unsafe { ptr::NonNull::new_unchecked(handle as *mut _) };

        Ok(File(handle))
    }

    /// Reads from the file into a byte buffer, until the buffer is filled or
    /// the end of the file is reached.
    /// Returns the number of bytes which were _not_ read into the buffer
    /// (e.g. 0 if buffer is filled, length of buffer is file is already at EOF).
    ///
    /// Use `seek` to set the file's position explicitly. Otherwise, this function
    /// will start reading after the end of the previous read or write.
    ///
    /// No buffering is performed by this function.
    pub fn read(&mut self, buf: &mut [u8]) -> usize {
        let arg_block = [
            self.0.as_ptr() as usize,
            buf.as_mut_ptr() as usize,
            buf.len(),
        ];

        unsafe { Op::Read.call(arg_block.as_ptr() as usize) }
    }

    /// Writes the contents of a buffer to a specified file at the current file position.
    /// Returns the number of bytes which were _not_ written, or 0 if all bytes
    /// were successfully written.
    ///
    /// Use `seek` to set the file's position explicitly. Otherwise, this function
    /// will start writing after the end of the previous read or write.
    ///
    /// No buffering is performed by this function.
    pub fn write(&mut self, buf: &[u8]) -> usize {
        let arg_block = [self.0.as_ptr() as usize, buf.as_ptr() as usize, buf.len()];

        unsafe { Op::Write.call(arg_block.as_ptr() as usize) }
    }

    /// Seeks to an offset from the beginning of the file.
    ///
    /// If `seek` fails, check `errno` for the detailed reason.
    pub fn seek(&mut self, offset: usize) -> Result<()> {
        let arg_block = [self.0.as_ptr() as usize, offset];

        let ret = unsafe { Op::Seek.call(arg_block.as_ptr() as usize) };

        if ret as isize >= 0 {
            Ok(())
        } else {
            Err(())
        }
    }

    /// Returns the length of a specified file.
    pub fn len(&self) -> Result<usize> {
        let ret = unsafe { Op::FLen.call(self.arg_block()) };

        check(ret)
    }

    /// Checks if this file is empty.
    pub fn is_empty(&self) -> bool {
        // If there's any issue with reading the length, assume it's empty
        let len = self.len().unwrap_or(0);
        len == 0
    }

    /// Returns true if this file is connected to a TTY.
    pub fn is_tty(&self) -> bool {
        let ret = unsafe { Op::IsTTY.call(self.arg_block()) };

        // Only possible error could be if we have an invalid file handle.
        ret == 1
    }

    /// Opens the host's standard input stream for reading.
    pub fn stdin() -> Result<File> {
        File::open(STDIO_PATH, OpenMode::ReadBinary)
    }

    /// Opens the host's standard output stream for writing.
    pub fn stdout() -> Result<File> {
        File::open(STDIO_PATH, OpenMode::WriteBinary)
    }

    /// Opens the host's standard error output stream for writing.
    ///
    /// Note that this is an optional extension, and it's not guaranteed the host
    /// debugger allows for separation of `stdout` / `stderr`.
    pub fn stderr() -> Result<File> {
        File::open(STDIO_PATH, OpenMode::AppendBinary)
    }

    /// Returns a pointer to an argument block which contains only the file handle.
    fn arg_block(&self) -> usize {
        let ptr = &self as *const _;
        ptr as usize
    }
}

impl Drop for File {
    fn drop(&mut self) {
        let ret = unsafe { Op::Close.call(self.arg_block()) };

        check(ret).expect("Failed to close file");
    }
}

impl fmt::Debug for File {
    fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
        let handle = self.0.as_ptr() as usize;

        fmt.debug_struct("File").field("handle", &handle).finish()
    }
}

impl fmt::Write for File {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        match self.write(s.as_bytes()) {
            0 => Ok(()),
            _ => Err(fmt::Error),
        }
    }
}

// Special path used for opening the standard I/O streams.
const STDIO_PATH: &str = ":tt\0";

/// Enum describing how the opened file can be used.
///
/// These match the file modes used by the C standard library function `fopen`.
#[repr(usize)]
pub enum OpenMode {
    /// `r`
    Read = 0,
    /// `rb`
    ReadBinary = 1,
    /// `r+`
    ReadWrite = 2,
    /// `r+b`
    ReadWriteBinary = 3,
    /// `w`
    Write = 4,
    /// `wb`
    WriteBinary = 5,
    /// `w+`
    WriteRead = 6,
    /// `w+b`
    WriteReadBinary = 7,
    /// `a`
    Append = 8,
    /// `ab`
    AppendBinary = 9,
    /// `a+`
    AppendRead = 10,
    /// `a+b`
    AppendReadBinary = 11,
}

/// Returns the path to a temporary file on the host computer.
///
/// The first parameter is a buffer into which the path will be written.
/// The second parameter is an identifier for this temporary file. Using the same
/// ID multiple times will give you the same temporary file.
///
/// The returned string is NUL-terminated, and can safely be used with other
/// semihosting functions.
///
/// The length of the buffer must be at least the value of the
/// `L_tmpnam` define of the host system.
pub fn tmp_name(buf: &mut [u8], id: u8) -> Result<&str> {
    let arg_block = [buf.as_mut_ptr() as usize, id as usize, buf.len()];

    let ret = unsafe { Op::TmpName.call(arg_block.as_ptr() as usize) };

    check(ret).and_then(move |_| core::str::from_utf8(buf).map_err(|_| ()))
}

/// Deletes a specified file on the host file system.
///
/// On error, returns a host-specific error code.
pub fn remove(path: &str) -> result::Result<(), usize> {
    assert_eq!(
        path.bytes().last().unwrap(),
        0,
        "Path must be NUL-terminated"
    );

    let arg_block = [path.as_ptr() as usize, path.len()];

    let ret = unsafe { Op::Remove.call(arg_block.as_ptr() as usize) };

    if ret == 0 {
        Ok(())
    } else {
        Err(ret)
    }
}

/// Renames a specified file on the host file system.
///
/// On error, returns a host-specific error code.
pub fn rename(old: &str, new: &str) -> result::Result<(), usize> {
    assert_eq!(
        old.bytes().last().unwrap(),
        0,
        "Old path must be NUL-terminated"
    );
    assert_eq!(new.bytes().last().unwrap(), 0, "New path be NUL-terminated");

    let arg_block = [
        old.as_ptr() as usize,
        old.len(),
        new.as_ptr() as usize,
        new.len(),
    ];

    let ret = unsafe { Op::Rename.call(arg_block.as_ptr() as usize) };

    if ret == 0 {
        Ok(())
    } else {
        Err(ret)
    }
}
