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

#![feature(globs)]
#![feature(macro_rules)]

extern crate gl;
extern crate native;
extern crate sb6;

use gl::types::*;
use std::ptr;
use vmath::Mat4;

mod vmath;

static VS_SRC: &'static str = "\
#version 330 core                                                  \n\
                                                                   \n\
layout (location = 0) in vec4 position;                            \n\
layout (location = 1) in vec3 normal;                              \n\
                                                                   \n\
out VS_OUT                                                         \n\
{                                                                  \n\
    vec3 normal;                                                   \n\
    vec4 color;                                                    \n\
} vs_out;                                                          \n\
                                                                   \n\
uniform mat4 mv_matrix;                                            \n\
uniform mat4 proj_matrix;                                          \n\
                                                                   \n\
void main(void)                                                    \n\
{                                                                  \n\
    gl_Position = proj_matrix * mv_matrix * position;              \n\
    vs_out.color = position * 2.0 + vec4(0.5, 0.5, 0.5, 0.0);      \n\
    vs_out.normal = normalize(mat3(mv_matrix) * normal);           \n\
}                                                                  \n\
";

static FS_SRC: &'static str = "\
#version 330 core                                                  \n\
                                                                   \n\
out vec4 color;                                                    \n\
                                                                   \n\
in VS_OUT                                                          \n\
{                                                                  \n\
    vec3 normal;                                                   \n\
    vec4 color;                                                    \n\
} fs_in;                                                           \n\
                                                                   \n\
void main(void)                                                    \n\
{                                                                  \n\
    color = vec4(1.0) * abs(normalize(fs_in.normal).z);            \n\
}                                                                  \n\
";

struct MyApp {
    info: sb6::AppInfo,
    program: GLuint,
    mv_location: GLint,
    proj_location: GLint,
    object: sb6::object::Object,
}

impl MyApp {
    fn new(init: sb6::AppInfo) -> MyApp {
        MyApp {
            info: init,
            program: 0,
            mv_location: -1,
            proj_location: -1,
            object: sb6::object::Object::new()
        }
    }
}

impl sb6::App for MyApp {
    fn get_app_info(&self) -> &sb6::AppInfo { &self.info }

    fn startup(&mut self) {
        self.program = gl::CreateProgram();

        let vs = gl::CreateShader(gl::VERTEX_SHADER);
        let fs = gl::CreateShader(gl::FRAGMENT_SHADER);

        unsafe {
            VS_SRC.with_c_str(
                |ptr| gl::ShaderSource(vs, 1, &ptr, ptr::null()));
            FS_SRC.with_c_str(
                |ptr| gl::ShaderSource(fs, 1, &ptr, ptr::null()));
        }

        gl::CompileShader(vs);
        gl::CompileShader(fs);

        sb6::shader::assert_compile_status(vs);
        sb6::shader::assert_compile_status(fs);

        gl::AttachShader(self.program, vs);
        gl::AttachShader(self.program, fs);
        gl::LinkProgram(self.program);
        sb6::program::assert_link_status(self.program);

        gl::DeleteShader(vs);
        gl::DeleteShader(fs);
        
        unsafe {
            self.mv_location = "mv_matrix".with_c_str(
                |ptr| gl::GetUniformLocation(self.program, ptr));
            self.proj_location = "proj_matrix".with_c_str(
                |ptr| gl::GetUniformLocation(self.program, ptr));
        }

        match self.object.load("media/objects/bunny_1k.sbm") {
            Ok(_) => (),
            e => fail!("failed to load sbm file: {}", e)
        }

        gl::Enable(gl::DEPTH_TEST);
        gl::DepthFunc(gl::LEQUAL);
    }

    fn shutdown(&mut self) {
        self.object.free();
        gl::DeleteProgram(self.program);
        self.mv_location = -1;
        self.proj_location = -1;
        self.program = 0;
    }

    fn render(&self, time: f64) {
        static black: [GLfloat, ..4] = [ 0.0, 0.0, 0.0, 1.0 ];
        static one: GLfloat = 1.0;
        let time = time as f32;

        let info = self.get_app_info();
        let aspect = info.window_width as f32 / info.window_height as f32;
        let proj_matrix =  Mat4::perspective(50.0, aspect, 0.1, 1000.0);
        let mv_matrix = Mat4::translate(0.0, 0.0, -3.0) *
            Mat4::rotate(time * 45.0, 0.0, 1.0, 0.0) *
            Mat4::rotate(time * 81.0, 1.0, 0.0, 0.0);

        unsafe {
            gl::Viewport(0, 0, info.window_width as i32,
                info.window_height as i32);
            gl::ClearBufferfv(gl::COLOR, 0, black.as_ptr());
            gl::ClearBufferfv(gl::DEPTH, 0, &one);

            gl::UseProgram(self.program);

            gl::UniformMatrix4fv(self.proj_location, 1, gl::FALSE,
                proj_matrix.as_ptr());

            gl::UniformMatrix4fv(self.mv_location, 1, gl::FALSE,
                mv_matrix.as_ptr());

            self.object.render();
        }
    }
}

fn main() {
    let mut init = sb6::AppInfo::default();
    init.title = "OpenGL SuperBible - Object Viewer";
    init.major_version = 3;
    init.minor_version = 3;
    let mut app = MyApp::new(init);
    sb6::run(&mut app);
}

#[start]
fn start(argc: int, argv: *const *const u8) -> int {
    native::start(argc, argv, main)
}

