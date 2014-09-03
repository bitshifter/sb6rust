/*
 * Copyright © 2012-2013 Graham Sellers
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

extern crate gl;

use gl::types::*;
use reader::BufferReader;
use std::io;
use std::mem;

#[deriving(Show)]
struct Header {
    gl_type: u32,
    gl_type_size: u32,
    gl_format: u32,
    gl_internal_format: u32,
    gl_base_internal_format: u32,
    pixel_width: u32,
    pixel_height: u32,
    pixel_depth: u32,
    array_elements: u32,
    faces: u32,
    mip_levels: u32,
    key_pair_bytes: u32
}

#[deriving(Clone, PartialEq, Show)]
pub enum LoadError {
    MagicError,
    IoError(io::IoErrorKind, &'static str),
}

fn io_error_to_error(io: io::IoError) -> LoadError {
    IoError(io.kind, io.desc)
}

macro_rules! read(
    ($e:expr) => (match $e { Ok(e) => e, Err(e) => return Err(io_error_to_error(e)) })
)

static IDENTIFIER: [u8, ..12] = 
    [ 0xAB, 0x4B, 0x54, 0x58, 0x20, 0x31, 0x31, 0xBB, 0x0D, 0x0A, 0x1A, 0x0A ];

pub fn load(filename: &str) -> Result<GLuint, LoadError> {
    let bytes = read!(io::File::open(&Path::new(filename)).read_to_end());
    let mut reader = BufferReader::new(bytes);
    let mut bytes_read = 0u;

    // check header magic
    let id = read!(reader.pop_slice::<u8>(IDENTIFIER.len()));
    if id != IDENTIFIER {
        println!("identifier: {} != {}", IDENTIFIER.as_slice(), id);
        return Err(MagicError)
    }
    bytes_read += id.len();

    // check endianness
    let endianness = read!(reader.pop_value::<u32>());
    if *endianness == 0x01020304 {
        // swap not impemented
        return Err(MagicError)
    }
    bytes_read += mem::size_of::<u32>();

    // read the rest of the header
    let header = read!(reader.pop_value::<Header>());
    bytes_read += mem::size_of::<u32>();

    println!("ktx header: {}", header);

    Ok(0)
}
