use std::io;
use std::mem;
use std::raw;

/// Reads from an owned byte vector.
/// This is similar to the built in std::io::MemReader, the main differences
/// being we are not concerned with endian conversion, and we don't copy the
/// data out of the buffer, we just return references to in the internal
/// buffer.
pub struct BufferReader {
    buf: Vec<u8>,
    pos: uint
}

impl BufferReader {
    pub fn new(buf: Vec<u8>) -> BufferReader {
        BufferReader {
            buf: buf,
            pos: 0
        }
    }

    /// Returns the buffer length
    pub fn len(&self) -> uint { self.buf.len() }

    /// Returns the number of bytes read from the buffer
    pub fn bytes_read(&self) -> uint { self.pos }

    /// Skip the given number of bytes
    pub fn skip_bytes(&mut self, bytes: uint) -> Result<(), io::IoError> {
        let skip_end = self.pos + bytes;
        if skip_end > self.buf.len() {
            return Err(io::standard_error(io::EndOfFile))
        }
        self.pos = skip_end;
        Ok(())
    }

    /// Pop a slice of T items
    pub fn pop_slice<'a, T>(&mut self, size: uint) -> Result<&'a [T], io::IoError> {
        let pop_bytes = mem::size_of::<T>() * size;
        let pop_end = self.pos + pop_bytes;
        if pop_end > self.buf.len() {
            return Err(io::standard_error(io::EndOfFile))
        }
        let ptr = unsafe { self.buf.as_ptr().offset(self.pos as int) };
        let out = unsafe { mem::transmute(
                raw::Slice { data: ptr, len: size } ) };
        self.pos = pop_end;
        Ok(out)
    }

    /// Pop a reference to T
    pub fn pop_value<'a, T>(&mut self) -> Result<&'a T, io::IoError> {
        let pop_end = self.pos + mem::size_of::<T>();
        if pop_end > self.buf.len() {
            return Err(io::standard_error(io::EndOfFile))
        }
        let ptr = unsafe { self.buf.as_ptr().offset(self.pos as int) };
        self.pos = pop_end;
        Ok(unsafe { &*(ptr as *const T) })
    }

    pub fn peek_slice<'a>(&'a self, start: uint, end: uint) -> Result<&'a [u8], io::IoError> {
        assert!(start <= end);
        if end > self.buf.len() {
            return Err(io::standard_error(io::EndOfFile))
        }
        Ok(unsafe {
            mem::transmute(
                raw::Slice {
                    data: self.buf.as_ptr().offset(start as int),
                    len: end - start
                })
        })
    }
}

