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
use std::ptr;

#[deriving(Clone, PartialEq, Show)]
pub enum ProgramError {
    ProgramInfoLog(String)
}

pub fn check_link_status(program: GLuint) -> Result<(), ProgramError> {
    unsafe {
        // Get the link status
        let mut status = gl::FALSE as GLint;
        gl::GetProgramiv(program, gl::LINK_STATUS, &mut status);

        // Fail on error
        if status != (gl::TRUE as GLint) {
            let mut len: GLint = 0;
            gl::GetProgramiv(program, gl::INFO_LOG_LENGTH, &mut len);
            // subtract 1 to skip the trailing null character
            let mut buf = Vec::from_elem(len as uint - 1, 0u8);
            gl::GetProgramInfoLog(program, len, ptr::null_mut(),
                buf.as_mut_ptr() as *mut GLchar);
            return Err(ProgramInfoLog(String::from_utf8(buf).unwrap_or(
                String::from_str("ProgramInfoLog not valid utf8"))));
        }
    }
    Ok(())
}

pub fn link_from_shaders(shaders: &[GLuint]) -> Result<GLuint, ProgramError> {
    let program = gl::CreateProgram();

    for shader in shaders.iter() {
        gl::AttachShader(program, *shader);
    }

    gl::LinkProgram(program);
    try!(check_link_status(program));

    for shader in shaders.iter() {
        gl::DeleteShader(*shader);
    }

    Ok(program)
}

#[deriving(Clone, PartialEq, Show)]
pub enum UniformError {
    UniformNotFound(GLuint, String, GLint)
}

pub fn get_uniform_location(program: GLuint, name: &str) -> Result<GLint, UniformError> {
    let result = name.with_c_str(|ptr| unsafe {
        gl::GetUniformLocation(program, ptr)
    });
    if result >= 0 {
        Ok(result)
    }
    else {
        Err(UniformNotFound(program, String::from_str(name), result))
    }
}
