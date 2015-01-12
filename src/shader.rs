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

#![allow(unstable)]

extern crate gl;

use gl::types::*;
use std::ffi;
use std::io;
use std::iter;
use std::ptr;


#[derive(Clone, PartialEq, Show)]
pub enum ShaderError {
    ShaderInfoLog(String),
}

#[derive(Clone, PartialEq, Show)]
pub enum LoadError {
    CompileError(String),
    IoError(io::IoErrorKind, &'static str),
}

pub fn check_compile_status(shader: GLuint) -> Result<(), ShaderError> {
    unsafe {
        // Get the compile status
        let mut status = gl::FALSE as GLint;
        gl::GetShaderiv(shader, gl::COMPILE_STATUS, &mut status);

        // Fail on error
        if status != (gl::TRUE as GLint) {
            let mut len = 0;
            gl::GetShaderiv(shader, gl::INFO_LOG_LENGTH, &mut len);
            // subtract 1 to skip the trailing null character
            let mut buf: Vec<u8> = iter::repeat(0u8).take(len as usize - 1).collect();
            gl::GetShaderInfoLog(shader, len, ptr::null_mut(),
                buf.as_mut_ptr() as *mut GLchar);
            return Err(ShaderError::ShaderInfoLog(String::from_utf8(buf).unwrap_or(
                String::from_str("ShaderInfoLog not valid utf8"))));
        }
    }
    Ok(())
}

pub fn create_from_source(src: &str, shader_type: GLenum) -> Result<GLuint, ShaderError> {
    unsafe {
        let result = gl::CreateShader(shader_type);
        gl::ShaderSource(result, 1, &ffi::CString::from_slice(src.as_bytes()).as_ptr(), ptr::null());
        gl::CompileShader(result);
        match check_compile_status(result) {
            Ok(_) => Ok(result),
            Err(e) => Err(e)
        }
    }
}

pub fn load(filename: &str, shader_type: GLenum) -> Result<GLuint, LoadError> {
    let src = match io::File::open(&Path::new(filename)).read_to_string() {
        Ok(src) => src,
        Err(io) => return Err(LoadError::IoError(io.kind, io.desc))
    };

    match create_from_source(&src[], shader_type) {
        Ok(result) => Ok(result),
        Err(ShaderError::ShaderInfoLog(msg)) => Err(LoadError::CompileError(msg))
    }
}
