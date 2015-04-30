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
#[macro_use]
extern crate sb6;

use gl::types::*;
use vmath::Mat4;

mod vmath;

const VS_SRC: &'static str = "\
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

const FS_SRC: &'static str = "\
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

struct SampleApp {
    info: sb6::AppInfo,
    program: GLuint,
    mv_location: GLint,
    proj_location: GLint,
    object: sb6::object::Object,
}

impl SampleApp {
    fn new(init: sb6::AppInfo) -> SampleApp {
        SampleApp {
            info: init,
            program: 0,
            mv_location: -1,
            proj_location: -1,
            object: sb6::object::Object::new()
        }
    }
}

impl sb6::App for SampleApp {
    fn get_app_info(&self) -> &sb6::AppInfo { &self.info }

    fn startup(&mut self) {
        unsafe {
            self.program = gl::CreateProgram();

            let vs = sb6::shader::create_from_source(VS_SRC, gl::VERTEX_SHADER).unwrap();
            let fs = sb6::shader::create_from_source(FS_SRC, gl::FRAGMENT_SHADER).unwrap();

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

            load_object_or_panic!(&mut self.object, "media/objects/bunny_1k.sbm");

            gl::Enable(gl::DEPTH_TEST);
            gl::DepthFunc(gl::LEQUAL);
        }
    }

    fn shutdown(&mut self) {
        self.object.free();
        unsafe { gl::DeleteProgram(self.program); }
        self.mv_location = -1;
        self.proj_location = -1;
        self.program = 0;
    }

    fn render(&self, time: f64) {
        const BLACK: [GLfloat; 4] = [ 0.0, 0.0, 0.0, 1.0 ];
        const ONE: GLfloat = 1.0;
        let time = time as f32;

        let aspect = self.info.window_width as f32 / self.info.window_height as f32;
        let proj_matrix =  Mat4::perspective(50.0, aspect, 0.1, 1000.0);
        let mv_matrix = Mat4::translate(0.0, 0.0, -3.0) *
            Mat4::rotate(time * 45.0, 0.0, 1.0, 0.0) *
            Mat4::rotate(time * 81.0, 1.0, 0.0, 0.0);

        unsafe {
            gl::Viewport(0, 0, self.info.window_width as i32,
                self.info.window_height as i32);
            gl::ClearBufferfv(gl::COLOR, 0, BLACK.as_ptr());
            gl::ClearBufferfv(gl::DEPTH, 0, &ONE);

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
    let mut app = SampleApp::new(init);
    sb6::run(&mut app);
}

