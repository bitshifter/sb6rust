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

#![feature(globs)]

extern crate gl;
extern crate native;
extern crate sb6;

use gl::types::*;
use std::ptr;

const VS_SRC: &'static str = "\
#version 410 core                                                 \n\
                                                                  \n\
void main(void)                                                   \n\
{                                                                 \n\
    const vec4 vertices[] = vec4[](vec4( 0.25, -0.25, 0.5, 1.0),  \n\
                                   vec4(-0.25, -0.25, 0.5, 1.0),  \n\
                                   vec4( 0.25,  0.25, 0.5, 1.0)); \n\
                                                                  \n\
    gl_Position = vertices[gl_VertexID];                          \n\
}                                                                 \n\
";

const TCS_SRC: &'static str = "\
#version 410 core                                                                 \n\
                                                                                  \n\
layout (vertices = 3) out;                                                        \n\
                                                                                  \n\
void main(void)                                                                   \n\
{                                                                                 \n\
    if (gl_InvocationID == 0)                                                     \n\
    {                                                                             \n\
        gl_TessLevelInner[0] = 5.0;                                               \n\
        gl_TessLevelOuter[0] = 5.0;                                               \n\
        gl_TessLevelOuter[1] = 5.0;                                               \n\
        gl_TessLevelOuter[2] = 5.0;                                               \n\
    }                                                                             \n\
    gl_out[gl_InvocationID].gl_Position = gl_in[gl_InvocationID].gl_Position;     \n\
}                                                                                 \n\
";

const TES_SRC: &'static str = "\
#version 410 core                                                                 \n\
                                                                                  \n\
layout (triangles, equal_spacing, cw) in;                                         \n\
                                                                                  \n\
void main(void)                                                                   \n\
{                                                                                 \n\
    gl_Position = (gl_TessCoord.x * gl_in[0].gl_Position) +                       \n\
                  (gl_TessCoord.y * gl_in[1].gl_Position) +                       \n\
                  (gl_TessCoord.z * gl_in[2].gl_Position);                        \n\
}                                                                                 \n\
";

const GS_SRC: &'static str = "\
#version 410 core                                                                  \n\
                                                                                   \n\
layout (triangles) in;                                                             \n\
layout (points, max_vertices = 3) out;                                             \n\
                                                                                   \n\
void main(void)                                                                    \n\
{                                                                                  \n\
    int i;                                                                         \n\
                                                                                   \n\
    for (i = 0; i < gl_in.length(); i++)                                           \n\
    {                                                                              \n\
        gl_Position = gl_in[i].gl_Position;                                        \n\
        EmitVertex();                                                              \n\
    }                                                                              \n\
}                                                                                  \n\
";

const FS_SRC: &'static str = "\
#version 410 core                                                 \n\
                                                                  \n\
out vec4 color;                                                   \n\
                                                                  \n\
void main(void)                                                   \n\
{                                                                 \n\
    color = vec4(0.0, 0.8, 1.0, 1.0);                             \n\
}                                                                 \n\
";

struct MyApp {
    info: sb6::AppInfo,
    program: GLuint,
    vao: GLuint
}

impl MyApp {
    fn new(init: sb6::AppInfo) -> MyApp {
        MyApp { info: init, program: 0, vao: 0 }
    }
}

impl sb6::App for MyApp {
    fn get_app_info(&self) -> &sb6::AppInfo { &self.info }

    fn startup(&mut self) {
        self.program = gl::CreateProgram();

        let vs = gl::CreateShader(gl::VERTEX_SHADER);
        let tcs = gl::CreateShader(gl::TESS_CONTROL_SHADER);
        let tes = gl::CreateShader(gl::TESS_EVALUATION_SHADER);
        let gs = gl::CreateShader(gl::GEOMETRY_SHADER);
        let fs = gl::CreateShader(gl::FRAGMENT_SHADER);

        unsafe {
            VS_SRC.with_c_str(
                |ptr| gl::ShaderSource(vs, 1, &ptr, ptr::null()));
            gl::CompileShader(vs);
            sb6::shader::check_compile_status(vs).unwrap();

            TCS_SRC.with_c_str(
                |ptr| gl::ShaderSource(tcs, 1, &ptr, ptr::null()));
            gl::CompileShader(tcs);
            sb6::shader::check_compile_status(tcs).unwrap();

            TES_SRC.with_c_str(
                |ptr| gl::ShaderSource(tes, 1, &ptr, ptr::null()));
            gl::CompileShader(tes);
            sb6::shader::check_compile_status(tes).unwrap();

            GS_SRC.with_c_str(
                |ptr| gl::ShaderSource(gs, 1, &ptr, ptr::null()));
            gl::CompileShader(gs);
            sb6::shader::check_compile_status(gs).unwrap();

            FS_SRC.with_c_str(
                |ptr| gl::ShaderSource(fs, 1, &ptr, ptr::null()));
            gl::CompileShader(fs);
            sb6::shader::check_compile_status(fs).unwrap();
        }

        gl::AttachShader(self.program, vs);
        gl::AttachShader(self.program, tcs);
        gl::AttachShader(self.program, tes);
        gl::AttachShader(self.program, gs);
        gl::AttachShader(self.program, fs);

        gl::LinkProgram(self.program);
        sb6::program::check_link_status(self.program).unwrap();

        gl::DeleteShader(vs);
        gl::DeleteShader(tcs);
        gl::DeleteShader(tes);
        gl::DeleteShader(gs);
        gl::DeleteShader(fs);

        unsafe {
            gl::GenVertexArrays(1, &mut self.vao);
        }
        gl::BindVertexArray(self.vao);

        gl::PolygonMode(gl::FRONT_AND_BACK, gl::LINE);
    }

    fn shutdown(&mut self) {
        unsafe {
            gl::DeleteVertexArrays(1, &self.vao);
        }
        gl::DeleteProgram(self.program);
        self.vao = 0;
        self.program = 0;
    }

    fn render(&self, _: f64) {
        const GREEN: [GLfloat, ..4] = [ 0.0, 0.25, 0.0, 1.0 ];
        unsafe {
            gl::ClearBufferfv(gl::COLOR, 0, GREEN.as_ptr());
        }

        gl::UseProgram(self.program);
        gl::PointSize(5.0);
        gl::DrawArrays(gl::PATCHES, 0, 3);
    }
}

fn main() {
    let mut init = sb6::AppInfo::default();
    init.title = "OpenGL SuperBible - Tessellation and Geometry Shaders";
    let mut app = MyApp::new(init);
    sb6::run(&mut app);
}

#[start]
fn start(argc: int, argv: *const *const u8) -> int {
    native::start(argc, argv, main)
}

