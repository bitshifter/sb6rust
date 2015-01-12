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
extern crate sb6;

use gl::types::*;
use std::mem;
use std::num::Float;
use std::ptr;
use vmath::Mat4;

mod vmath;

const VERTEX_POSITIONS: [GLfloat; 108] = [
    -0.25,  0.25, -0.25,
    -0.25, -0.25, -0.25,
     0.25, -0.25, -0.25,

     0.25, -0.25, -0.25,
     0.25,  0.25, -0.25,
    -0.25,  0.25, -0.25,

     0.25, -0.25, -0.25,
     0.25, -0.25,  0.25,
     0.25,  0.25, -0.25,

     0.25, -0.25,  0.25,
     0.25,  0.25,  0.25,
     0.25,  0.25, -0.25,

     0.25, -0.25,  0.25,
    -0.25, -0.25,  0.25,
     0.25,  0.25,  0.25,

    -0.25, -0.25,  0.25,
    -0.25,  0.25,  0.25,
     0.25,  0.25,  0.25,

    -0.25, -0.25,  0.25,
    -0.25, -0.25, -0.25,
    -0.25,  0.25,  0.25,

    -0.25, -0.25, -0.25,
    -0.25,  0.25, -0.25,
    -0.25,  0.25,  0.25,

    -0.25, -0.25,  0.25,
     0.25, -0.25,  0.25,
     0.25, -0.25, -0.25,

     0.25, -0.25, -0.25,
    -0.25, -0.25, -0.25,
    -0.25, -0.25,  0.25,

    -0.25,  0.25, -0.25,
     0.25,  0.25, -0.25,
     0.25,  0.25,  0.25,

     0.25,  0.25,  0.25,
    -0.25,  0.25,  0.25,
    -0.25,  0.25, -0.25
];

const VS_SRC: &'static str = "\
#version 330 core                                                  \n
                                                                   \n
in vec4 position;                                                  \n
                                                                   \n
out VS_OUT                                                         \n
{                                                                  \n
    vec4 color;                                                    \n
} vs_out;                                                          \n
                                                                   \n
uniform mat4 mv_matrix;                                            \n
uniform mat4 proj_matrix;                                          \n
                                                                   \n
void main(void)                                                    \n
{                                                                  \n
    gl_Position = proj_matrix * mv_matrix * position;              \n
    vs_out.color = position * 2.0 + vec4(0.5, 0.5, 0.5, 0.0);      \n
}                                                                  \n
";

const FS_SRC: &'static str = "\
#version 330 core                                                  \n\
                                                                   \n\
out vec4 color;                                                    \n\
                                                                   \n\
in VS_OUT                                                          \n\
{                                                                  \n\
    vec4 color;                                                    \n\
} fs_in;                                                           \n\
                                                                   \n\
void main(void)                                                    \n\
{                                                                  \n\
    color = fs_in.color;                                           \n\
}                                                                  \n\
";

struct MyApp {
    info: sb6::AppInfo,
    program: GLuint,
    vao: GLuint,
    buffer: GLuint,
    mv_location: GLint,
    proj_location: GLint,
    proj_matrix: Mat4
}

impl MyApp {
    fn new(init: sb6::AppInfo) -> MyApp {
        MyApp {
            info: init,
            program: 0,
            vao: 0,
            buffer: 0,
            mv_location: -1,
            proj_location: -1,
            proj_matrix: Mat4::identity()
        }
    }

    fn update_proj_matrix(&mut self) {
        let aspect = self.info.window_width as f32 / self.info.window_height as f32;
        self.proj_matrix = Mat4::perspective(50.0, aspect, 0.1, 1000.0);
    }
}

impl sb6::App for MyApp {
    fn get_app_info(&self) -> &sb6::AppInfo { &self.info }
    fn startup(&mut self) {
        unsafe {
            self.program = gl::CreateProgram();

            let fs = sb6::shader::create_from_source(FS_SRC, gl::FRAGMENT_SHADER).unwrap();
            let vs = sb6::shader::create_from_source(VS_SRC, gl::VERTEX_SHADER).unwrap();

            gl::AttachShader(self.program, vs);
            gl::AttachShader(self.program, fs);
            gl::LinkProgram(self.program);
            sb6::program::check_link_status(self.program).unwrap();

            gl::DeleteShader(vs);
            gl::DeleteShader(fs);

            self.mv_location = sb6::program::get_uniform_location(
                self.program, "mv_matrix").unwrap();
            self.proj_location = sb6::program::get_uniform_location(
                self.program, "proj_matrix").unwrap();

            gl::GenVertexArrays(1, &mut self.vao);
            gl::BindVertexArray(self.vao);

            gl::GenBuffers(1, &mut self.buffer);
            gl::BindBuffer(gl::ARRAY_BUFFER, self.buffer);
            gl::BufferData(gl::ARRAY_BUFFER,
                           mem::size_of_val(&VERTEX_POSITIONS) as GLsizeiptr,
                           mem::transmute(VERTEX_POSITIONS.as_ptr()),
                           gl::STATIC_DRAW);
            gl::VertexAttribPointer(0, 3, gl::FLOAT, gl::FALSE, 0, ptr::null());
            gl::EnableVertexAttribArray(0);

            gl::Enable(gl::CULL_FACE);
            gl::FrontFace(gl::CW);

            gl::Enable(gl::DEPTH_TEST);
            gl::DepthFunc(gl::LEQUAL);
        }
        self.update_proj_matrix();
    }

    fn shutdown(&mut self) {
        unsafe {
            gl::DeleteVertexArrays(1, &self.vao);
            gl::DeleteBuffers(1, &self.buffer);
            gl::DeleteProgram(self.program);
        }
        self.mv_location = -1;
        self.proj_location = -1;
        self.buffer = 0;
        self.vao = 0;
        self.program = 0;
    }

    fn on_resize(&mut self, width: isize, height: isize) {
        self.info.window_width = width as u32;
        self.info.window_height = height as u32;
        self.update_proj_matrix();
    }

    fn render(&self, time: f64) {
        const GREEN: [GLfloat; 4] = [ 0.0, 0.25, 0.0, 1.0 ];
        const ONE: GLfloat = 1.0;

        unsafe {
            gl::Viewport(0, 0, self.info.window_width as i32,
                         self.info.window_height as i32);

            gl::ClearBufferfv(gl::COLOR, 0, GREEN.as_ptr());
            gl::ClearBufferfv(gl::DEPTH, 0, &ONE);

            gl::UseProgram(self.program);

            gl::UniformMatrix4fv(self.proj_location, 1, gl::FALSE,
                                 self.proj_matrix.as_ptr());

            let f = time as f32 * 0.3;
            let mv_matrix =
                Mat4::translate(0.0, 0.0, -4.0) *
                Mat4::translate((2.1 * f).sin() * 0.5,
                (1.7 * f).cos() * 0.5,
                (1.3 * f).sin() * (1.5 * f).cos() * 2.0) *
                Mat4::rotate(time as f32 * 45.0, 0.0, 1.0, 0.0) *
                Mat4::rotate(time as f32 * 81.0, 1.0, 0.0, 0.0);
            gl::UniformMatrix4fv(self.mv_location, 1, gl::FALSE,
                                 mv_matrix.as_ptr());

            gl::DrawArrays(gl::TRIANGLES, 0, 36);
        }
    }
}

fn main() {
    let mut init = sb6::AppInfo::default();
    init.title = "OpenGL SuperBible - Moving Triangle";
    let mut app = MyApp::new(init);
    sb6::run(&mut app);
}

