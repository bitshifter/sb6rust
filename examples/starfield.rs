/*
 * Copyright (c) 2012-2013 Graham Sellers
 * Copyright (c) 2015 Cameron Hart
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
extern crate rand;
extern crate sb6;

use gl::types::*;
use rand::{ Rng };
use sb6::vmath;
use std::mem;
use std::ptr;
use std::slice;

const NUM_STARS: usize = 2000;

const FS_SRC: &'static str = "\
#version 410 core                                              \n
                                                               \n
layout (location = 0) out vec4 color;                          \n
                                                               \n
uniform sampler2D tex_star;                                    \n
flat in vec4 starColor;                                        \n
                                                               \n
void main(void)                                                \n
{                                                              \n
    color = starColor * texture(tex_star, gl_PointCoord);      \n
}                                                              \n
";

const VS_SRC: &'static str = "\
#version 410 core                                              \n
                                                               \n
layout (location = 0) in vec4 position;                        \n
layout (location = 1) in vec4 color;                           \n
                                                               \n
uniform float time;                                            \n
uniform mat4 proj_matrix;                                      \n
                                                               \n
flat out vec4 starColor;                                       \n
                                                               \n
void main(void)                                                \n
{                                                              \n
    vec4 newVertex = position;                                 \n
                                                               \n
    newVertex.z += time;                                       \n
    newVertex.z = fract(newVertex.z);                          \n
                                                               \n
    float size = (20.0 * newVertex.z * newVertex.z);           \n
                                                               \n
    starColor = smoothstep(1.0, 7.0, size) * color;            \n
                                                               \n
    newVertex.z = (999.9 * newVertex.z) - 1000.0;              \n
    gl_Position = proj_matrix * newVertex;                     \n
    gl_PointSize = size;                                       \n
}                                                              \n
";

struct Star {
    position: vmath::Vec3,
    color: vmath::Vec3
}

struct Uniforms {
    time: GLint,
    proj_matrix: GLint
}

struct SampleApp {
    info: sb6::AppInfo,
    render_prog: GLuint,
    star_texture: GLuint,
    star_vao: GLuint,
    star_buffer: GLuint,
    uniforms: Uniforms
}

impl SampleApp {
    fn new(init: sb6::AppInfo) -> SampleApp {
        SampleApp {
            info: init,
            render_prog: 0,
            star_texture: 0,
            star_vao: 0,
            star_buffer: 0,
            uniforms: Uniforms {
                time: -1,
                proj_matrix: -1
            }
        }
    }
}

impl sb6::App for SampleApp {
    fn get_app_info(&self) -> &sb6::AppInfo { &self.info }

    fn startup(&mut self) {
        let fs = sb6::shader::create_from_source(FS_SRC, gl::FRAGMENT_SHADER).unwrap();
        let vs = sb6::shader::create_from_source(VS_SRC, gl::VERTEX_SHADER).unwrap();

        unsafe {
            self.render_prog = gl::CreateProgram();
            gl::AttachShader(self.render_prog, vs);
            gl::AttachShader(self.render_prog, fs);
            gl::LinkProgram(self.render_prog);
            sb6::program::check_link_status(self.render_prog).unwrap();

            gl::DeleteShader(vs);
            gl::DeleteShader(fs);
        }

        self.uniforms.time = sb6::program::get_uniform_location(
            self.render_prog, "time").unwrap();
        self.uniforms.proj_matrix = sb6::program::get_uniform_location(
            self.render_prog, "proj_matrix").unwrap();

        self.star_texture = sb6::ktx::load("media/textures/star.ktx").unwrap();

        unsafe {
            gl::GenVertexArrays(1, &mut self.star_vao);
            gl::BindVertexArray(self.star_vao);

            gl::GenBuffers(1, &mut self.star_buffer);
            gl::BindBuffer(gl::ARRAY_BUFFER, self.star_buffer);
            gl::BufferData(gl::ARRAY_BUFFER, (NUM_STARS * mem::size_of::<Star>()) as GLsizeiptr,
                ptr::null(), gl::STATIC_DRAW);
        }

        let stars = unsafe {
            slice::from_raw_parts_mut(
                gl::MapBufferRange(gl::ARRAY_BUFFER, 0,
                   (NUM_STARS * mem::size_of::<Star>()) as GLsizeiptr,
                   gl::MAP_WRITE_BIT | gl::MAP_INVALIDATE_BUFFER_BIT) as *mut Star,
                   NUM_STARS)
        };

        let mut rng = rand::weak_rng();

        for star in &mut stars[..] {
            star.position.x = (rng.gen::<f32>() * 2.0 - 1.0) * 100.0;
            star.position.y = (rng.gen::<f32>() * 2.0 - 1.0) * 100.0;
            star.position.z = rng.gen::<f32>();
            star.color.x = 0.8 + rng.gen::<f32>() * 0.2;
            star.color.y = 0.8 + rng.gen::<f32>() * 0.2;
            star.color.z = 0.8 + rng.gen::<f32>() * 0.2;
        }

        unsafe {
            gl::UnmapBuffer(gl::ARRAY_BUFFER);

            gl::VertexAttribPointer(0, 3, gl::FLOAT, gl::FALSE, mem::size_of::<Star>() as GLint,
                ptr::null());
            gl::VertexAttribPointer(1, 3, gl::FLOAT, gl::FALSE, mem::size_of::<Star>() as GLint,
                mem::transmute(mem::size_of::<vmath::Vec3>()));
            gl::EnableVertexAttribArray(0);
            gl::EnableVertexAttribArray(1);
        }
    }

    fn render(&mut self, time: f64) {
        const BLACK: [GLfloat; 4] = [ 0.0, 0.0, 0.0, 0.0 ];
        const ONE: [GLfloat; 1] = [ 1.0 ];

        let proj_matrix = vmath::perspective(
            50.0, self.info.window_width as f32 / self.info.window_height as f32, 0.1, 1000.0);

        let mut t = time as f32;
        t *= 0.1;
        t -= t.floor();

        unsafe {
            gl::Viewport(0, 0, self.info.window_width as GLint, self.info.window_height as GLint);
            gl::ClearBufferfv(gl::COLOR, 0, BLACK.as_ptr());
            gl::ClearBufferfv(gl::DEPTH, 0, ONE.as_ptr());

            gl::UseProgram(self.render_prog);

            gl::Uniform1f(self.uniforms.time, t);
            gl::UniformMatrix4fv(self.uniforms.proj_matrix, 1, gl::FALSE, proj_matrix.as_ptr());

            gl::Enable(gl::BLEND);
            gl::BlendFunc(gl::ONE, gl::ONE);

            gl::BindVertexArray(self.star_vao);

            gl::Enable(gl::PROGRAM_POINT_SIZE);
            gl::DrawArrays(gl::POINTS, 0, NUM_STARS as GLint);
        }
    }
}

fn main() {
    let init = sb6::AppInfo {
        title: "OpenGL SuperBible - Starfield",
        ..
        sb6::AppInfo::default()
    };
    let mut app = SampleApp::new(init);
    sb6::run(&mut app);
}

