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

const VS_SRC: &str = r"
#version 410 core

void main(void)
{
    const vec4 vertices[] = vec4[](vec4( 0.25, -0.25, 0.5, 1.0),
    vec4(-0.25, -0.25, 0.5, 1.0),
    vec4( 0.25,  0.25, 0.5, 1.0));

    gl_Position = vertices[gl_VertexID];
}
";

const TCS_SRC: &str = r"
#version 410 core

layout (vertices = 3) out;

void main(void)
{
    if (gl_InvocationID == 0)
    {
        gl_TessLevelInner[0] = 5.0;
        gl_TessLevelOuter[0] = 5.0;
        gl_TessLevelOuter[1] = 5.0;
        gl_TessLevelOuter[2] = 5.0;
    }
    gl_out[gl_InvocationID].gl_Position = gl_in[gl_InvocationID].gl_Position;
}
";

const TES_SRC: &str = r"
#version 410 core

layout (triangles, equal_spacing, cw) in;

void main(void)
{
    gl_Position = (gl_TessCoord.x * gl_in[0].gl_Position) +
        (gl_TessCoord.y * gl_in[1].gl_Position) +
        (gl_TessCoord.z * gl_in[2].gl_Position);
}
";

const GS_SRC: &str = r"
#version 410 core

layout (triangles) in;
layout (points, max_vertices = 3) out;

void main(void)
{
    int i;

    for (i = 0; i < gl_in.length(); i++)
    {
        gl_Position = gl_in[i].gl_Position;
        EmitVertex();
    }
}
";

const FS_SRC: &str = r"
#version 410 core

out vec4 color;

void main(void)
{
    color = vec4(0.0, 0.8, 1.0, 1.0);
}
";

struct SampleApp {
    info: sb6::AppInfo,
    program: GLuint,
    vao: GLuint,
}

impl SampleApp {
    fn new(init: sb6::AppInfo) -> SampleApp {
        SampleApp {
            info: init,
            program: 0,
            vao: 0,
        }
    }
}

impl sb6::App for SampleApp {
    fn get_app_info(&self) -> &sb6::AppInfo {
        &self.info
    }

    fn startup(&mut self) {
        unsafe {
            self.program = gl::CreateProgram();

            let vs = sb6::shader::create_from_source(VS_SRC, gl::VERTEX_SHADER).unwrap();
            let tcs = sb6::shader::create_from_source(TCS_SRC, gl::TESS_CONTROL_SHADER).unwrap();
            let tes = sb6::shader::create_from_source(TES_SRC, gl::TESS_EVALUATION_SHADER).unwrap();
            let gs = sb6::shader::create_from_source(GS_SRC, gl::GEOMETRY_SHADER).unwrap();
            let fs = sb6::shader::create_from_source(FS_SRC, gl::FRAGMENT_SHADER).unwrap();

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

            gl::GenVertexArrays(1, &mut self.vao);
            gl::BindVertexArray(self.vao);

            gl::PolygonMode(gl::FRONT_AND_BACK, gl::LINE);
        }
    }

    fn shutdown(&mut self) {
        unsafe {
            gl::DeleteVertexArrays(1, &self.vao);
            gl::DeleteProgram(self.program);
        }
        self.vao = 0;
        self.program = 0;
    }

    fn render(&mut self, _: f64) {
        const GREEN: [GLfloat; 4] = [0.0, 0.25, 0.0, 1.0];
        unsafe {
            gl::ClearBufferfv(gl::COLOR, 0, GREEN.as_ptr());
            gl::UseProgram(self.program);
            gl::PointSize(5.0);
            gl::DrawArrays(gl::PATCHES, 0, 3);
        }
    }
}

fn main() {
    let mut init = sb6::AppInfo::default();
    init.title = "OpenGL SuperBible - Tessellation and Geometry Shaders";
    let mut app = SampleApp::new(init);
    sb6::run(&mut app);
}
