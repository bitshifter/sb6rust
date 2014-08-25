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

struct Header {
    identifier: [u8, ..12],
    endianness: u32,
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

#[deriving(Show)]
pub enum KtxError {
    FileError,
    MagicError,
    SizeError,
}

pub fn load(filename: &str) -> Result<GLuint, KtxError> {
    let bytes = match io::File::open(&Path::new(filename)).read_to_end() {
        Ok(v) => v,
        Err(_) => return Err(FileError)
    };
    //let reader = BufferReader::new(bytes);
    Err(MagicError)
}
