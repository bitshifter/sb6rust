/*
 * Copyright (c) 2012-2013 Graham Sellers
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

extern crate gl;

use gl::types::*;
use reader::BufferReader;
use std::fs;
use std::io;
use std::io::Read;
use std::mem;
use std::path::Path;
use std::str;

#[derive(Debug)]
struct Header {
    gl_type: u32,
    gl_type_size: u32,
    gl_format: u32,
    gl_internal_format: u32,
    gl_base_internal_format: u32,
    pixel_width: i32,
    pixel_height: i32,
    pixel_depth: i32,
    array_elements: i32,
    faces: i32,
    mip_levels: i32,
    key_pair_bytes: u32
}

#[derive(Debug)]
pub enum LoadError {
    MagicError,
    HeaderError,
    IoError(io::Error),
}

impl From<io::Error> for LoadError {
    fn from(e: io::Error) -> LoadError {
        LoadError::IoError(e)
    }
}

const IDENTIFIER: [u8; 12] =
    [ 0xAB, 0x4B, 0x54, 0x58, 0x20, 0x31, 0x31, 0xBB, 0x0D, 0x0A, 0x1A, 0x0A ];

fn calculate_stride(h: &Header, width: i32, pad: usize) -> Result<isize, LoadError> {
    let channels = match h.gl_base_internal_format {
        gl::RED => 1,
        gl::RG => 2,
        gl::BGR => 3,
        gl::RGB => 3,
        gl::BGRA => 4,
        gl::RGBA => 4,
        _ => return Err(LoadError::HeaderError)
    };
    Ok((((h.gl_type_size * channels * width as u32) as usize + ((pad - 1)) & !(pad - 1))) as isize)
}

fn calculate_face_size(h: &Header) -> Result<isize, LoadError> {
    let stride = try!(calculate_stride(h, h.pixel_width, 4));
    Ok(stride * h.pixel_height as isize)
}

pub fn load(filename: &str) -> Result<GLuint, LoadError> {
    let mut file = try!(fs::File::open(&Path::new(filename)));
    let mut bytes = Vec::new();
    try!(file.read_to_end(&mut bytes));
    let mut reader = BufferReader::new(bytes);

    // check header magic
    let id = try!(reader.pop_slice::<u8>(IDENTIFIER.len()));
    if id != IDENTIFIER {
        debug!("identifier: {} != {}", str::from_utf8(&IDENTIFIER).unwrap(),
            str::from_utf8(id).unwrap());
        return Err(LoadError::MagicError)
    }

    // check endianness
    let endianness = try!(reader.pop_value::<u32>());
    if *endianness == 0x01020304 {
        // swap not impemented
        return Err(LoadError::MagicError)
    }

    // read the rest of the header
    let h = try!(reader.pop_value::<Header>());

    // check for insanity
    if h.pixel_width == 0 || (h.pixel_height == 0 && h.pixel_depth != 0) {
        return Err(LoadError::HeaderError)
    }

    // guess the target (texture type)
    let target = if h.pixel_height == 0 {
        if h.array_elements == 0 {
            gl::TEXTURE_1D
        }
        else {
            gl::TEXTURE_1D_ARRAY
        }
    }
    else if h.pixel_depth == 0 {
        if h.array_elements == 0 {
            if h.faces == 0 {
                gl::TEXTURE_2D
            }
            else {
                gl::TEXTURE_CUBE_MAP
            }
        }
        else {
            if h.faces == 0 {
                gl::TEXTURE_2D_ARRAY
            }
            else {
                gl::TEXTURE_CUBE_MAP_ARRAY
            }
        }
    }
    else {
        gl::TEXTURE_3D
    };

    let mut tex:u32 = 0;
    unsafe {
        gl::GenTextures(1, &mut tex);
        gl::BindTexture(target, tex);
    }

    // skip unused key pair bytes
    try!(reader.skip_bytes(h.key_pair_bytes as usize));

    let data_size = reader.len() - reader.bytes_read();
    let data = try!(reader.pop_slice::<u8>(data_size));

    let mip_levels = match h.mip_levels {
        0 => 1,
        n => n
    };

    unsafe {
        match target {
            gl::TEXTURE_1D => {
                gl::TexStorage1D(gl::TEXTURE_1D, mip_levels,
                    h.gl_internal_format, h.pixel_width);
                gl::TexSubImage1D(gl::TEXTURE_1D, 0, 0, h.pixel_width,
                    h.gl_format, h.gl_internal_format,
                    mem::transmute(data.as_ptr()));
            },
            gl::TEXTURE_2D => {
                gl::TexStorage2D(gl::TEXTURE_2D, mip_levels,
                    h.gl_internal_format, h.pixel_width, h.pixel_height);
                let mut ptr = mem::transmute(data.as_ptr());
                let mut height = h.pixel_height;
                let mut width = h.pixel_width;
                gl::PixelStorei(gl::UNPACK_ALIGNMENT, 1);
                for i in (0..mip_levels) {
                    gl::TexSubImage2D(gl::TEXTURE_2D, i, 0, 0, width, height,
                        h.gl_format, h.gl_type, ptr);
                    let stride = try!(calculate_stride(h, width, 1));
                    ptr = ptr.offset(height as isize * stride);
                    height >>= 1;
                    width >>= 1;
                    if height == 0 {
                        height = 1;
                    }
                    if width == 0 {
                        width = 1;
                    }
                }
            },
            gl::TEXTURE_1D_ARRAY => {
                gl::TexStorage2D(gl::TEXTURE_1D_ARRAY, mip_levels,
                    h.gl_internal_format, h.pixel_width, h.array_elements);
                gl::TexSubImage2D(gl::TEXTURE_1D_ARRAY, 0, 0, 0, h.pixel_width,
                    h.array_elements, h.gl_format, h.gl_type,
                    mem::transmute(data.as_ptr()));
            }
            gl::TEXTURE_2D_ARRAY => {
                gl::TexStorage3D(gl::TEXTURE_2D_ARRAY, mip_levels,
                    h.gl_internal_format, h.pixel_width, h.pixel_height,
                    h.array_elements);
                gl::TexSubImage3D(gl::TEXTURE_2D_ARRAY, 0, 0, 0, 0,
                    h.pixel_width, h.pixel_height, h.array_elements,
                    h.gl_format, h.gl_type, mem::transmute(data.as_ptr()));
            },
            gl::TEXTURE_CUBE_MAP => {
                gl::TexStorage2D(gl::TEXTURE_CUBE_MAP, mip_levels,
                    h.gl_internal_format, h.pixel_width, h.pixel_height);
                let mut ptr = mem::transmute(data.as_ptr());
                let face_size = try!(calculate_face_size(h));
                for i in 0..h.faces as u32 {
                    gl::TexSubImage2D(gl::TEXTURE_CUBE_MAP_POSITIVE_X + i,
                        0, 0, 0, h.pixel_width, h.pixel_height, h.gl_format,
                        h.gl_type, ptr);
                    ptr = ptr.offset(face_size);
                }
            },
            gl::TEXTURE_CUBE_MAP_ARRAY => {
                gl::TexStorage3D(gl::TEXTURE_CUBE_MAP_ARRAY, mip_levels,
                    h.gl_internal_format, h.pixel_width, h.pixel_height,
                    h.array_elements);
                gl::TexSubImage3D(gl::TEXTURE_CUBE_MAP_ARRAY, 0, 0, 0, 0,
                    h.pixel_width, h.pixel_height, h.faces * h.array_elements,
                    h.gl_format, h.gl_type, mem::transmute(data.as_ptr()));
            },
            _ => return Err(LoadError::HeaderError)
        }
        if mip_levels == 1 {
            gl::GenerateMipmap(target);
        }
    }

    Ok(tex)
}
