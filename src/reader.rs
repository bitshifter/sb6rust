/*
 * Copyright (c) 2014 Cameron Hart
 *
 * Permission is hereby granted, free of charge, to any person obtaining a
 * copy of this software and associated documentation files (the "Software"),
 * to deal in the Software without restriction, including without limitation
 * the rights to use, copy, modify, merge, publish, distribute, sublicense,
 * and/or sell copies of the Software, and to permit persons to whom the
 * Software is furnished to do so, subject to the following conditions:
 *
 * The above copyright notice and this permission notice (including the next
 * paragraph) shall be included in all copies or substantial portions of the
 * Software.
 *
 * THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
 * IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
 * FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT.  IN NO EVENT SHALL
 * THE AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
 * LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING
 * FROM, OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER
 * DEALINGS IN THE SOFTWARE.
 */

use std::io;
use std::mem;
use std::slice;

/// Reads from an owned byte vector.
/// This is similar to the built in std::io::MemReader, the main differences
/// being we are not concerned with endian conversion, and we don't copy the
/// data out of the buffer, we just return references to in the internal
/// buffer.
pub struct BufferReader {
    buf: Vec<u8>,
    pos: usize
}

impl BufferReader {
    pub fn new(buf: Vec<u8>) -> BufferReader {
        BufferReader {
            buf: buf,
            pos: 0
        }
    }

    /// Returns the buffer length
    pub fn len(&self) -> usize { self.buf.len() }

    /// Returns the number of bytes read from the buffer
    pub fn bytes_read(&self) -> usize { self.pos }

    /// Skip the given number of bytes
    pub fn skip_bytes(&mut self, bytes: usize) -> Result<(), io::Error> {
        let skip_end = self.pos + bytes;
        if skip_end > self.buf.len() {
            return Err(io::Error::new(io::ErrorKind::InvalidInput, "Buffer overrun"))
        }
        self.pos = skip_end;
        Ok(())
    }

    /// Pop a slice of T items
    pub fn pop_slice<'a, T>(&mut self, size: usize) -> Result<&'a [T], io::Error> {
        let pop_bytes = mem::size_of::<T>() * size;
        let pop_end = self.pos + pop_bytes;
        if pop_end > self.buf.len() {
            return Err(io::Error::new(io::ErrorKind::InvalidInput, "Buffer overrun"))
        }
        let ptr = unsafe { self.buf.as_ptr().offset(self.pos as isize) as *const T };
        let out = unsafe { slice::from_raw_parts(ptr, size) };
        self.pos = pop_end;
        Ok(out)
    }

    /// Pop a reference to T
    pub fn pop_value<'a, T>(&mut self) -> Result<&'a T, io::Error> {
        let pop_end = self.pos + mem::size_of::<T>();
        if pop_end > self.buf.len() {
            return Err(io::Error::new(io::ErrorKind::InvalidInput, "Buffer overrun"))
        }
        let ptr = unsafe { self.buf.as_ptr().offset(self.pos as isize) };
        self.pos = pop_end;
        Ok(unsafe { &*(ptr as *const T) })
    }

    pub fn peek_slice<'a>(&'a self, start: usize, end: usize) -> Result<&'a [u8], io::Error> {
        assert!(start <= end);
        if end > self.buf.len() {
            return Err(io::Error::new(io::ErrorKind::InvalidInput, "Buffer overrun"))
        }
        Ok(unsafe {
            slice::from_raw_parts(self.buf.as_ptr().offset(start as isize), end - start)
        })
    }
}

