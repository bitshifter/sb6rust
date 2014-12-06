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
extern crate sb6;

use gl::types::*;
use std::mem;
use std::rand;
use std::rand::{ Rng };

mod vmath;

struct SamplePoints
{
    point: [vmath::Vec4, ..256],
    random_vectors: [vmath::Vec4, ..256]
}

struct RenderUniforms {
    mv_matrix: GLint,
    proj_matrix: GLint,
    shading_level: GLint,
}

impl RenderUniforms {
    fn new() -> RenderUniforms {
        RenderUniforms {
            mv_matrix: -1,
            proj_matrix: -1,
            shading_level: -1
        }
    }
}

struct SSAOUniforms {
    ssao_level: GLint,
    object_level: GLint,
    ssao_radius: GLint,
    randomize_points: GLint,
    point_count: GLint
}

impl SSAOUniforms {
    fn new() -> SSAOUniforms {
        SSAOUniforms {
            ssao_level: -1,
            object_level: -1,
            ssao_radius: -1,
            randomize_points: -1,
            point_count: -1
        }
    }
}

struct MyApp {
    info: sb6::AppInfo,
    render_program: GLuint,
    ssao_program: GLuint,
    render_fbo: GLuint,
    fbo_textures: [GLuint, ..3],
    quad_vao: GLuint,
    points_buffer: GLuint,
    object: sb6::object::Object,
    cube: sb6::object::Object,
    render: RenderUniforms,
    ssao: SSAOUniforms,
    last_time: f64,
    total_time: f64,
    point_count: u32,
    ssao_radius: f32,
    paused: bool,
    show_shading: bool,
    show_ao: bool,
    //weight_by_angle: bool,
    randomize_points: bool,
}

impl MyApp {
    fn new(init: sb6::AppInfo) -> MyApp {
        MyApp {
            info: init,
            render_program: 0,
            ssao_program: 0,
            render_fbo: 0,
            fbo_textures: [0, ..3],
            quad_vao: 0,
            points_buffer: 0,
            object: sb6::object::Object::new(),
            cube: sb6::object::Object::new(),
            render: RenderUniforms::new(),
            ssao: SSAOUniforms::new(),
            last_time: 0.0,
            total_time: 0.0,
            point_count: 10,
            ssao_radius: 0.05,
            paused: false,
            show_shading: true,
            show_ao: true,
            //weight_by_angle: true,
            randomize_points: true,
        }
    }

    fn load_shaders(&mut self) {
        if self.render_program != 0 {
            unsafe { gl::DeleteProgram(self.render_program); }
        }

        let render_shaders = [
            sb6::shader::load("media/shaders/ssao/render.vs.glsl",
                gl::VERTEX_SHADER).unwrap(),
            sb6::shader::load("media/shaders/ssao/render.fs.glsl",
                gl::FRAGMENT_SHADER).unwrap(),
            ];

        self.render_program = sb6::program::link_from_shaders(
            &render_shaders).unwrap();

        self.render.mv_matrix = sb6::program::get_uniform_location(
            self.render_program, "mv_matrix").unwrap();
        self.render.proj_matrix = sb6::program::get_uniform_location(
            self.render_program, "proj_matrix").unwrap();
        self.render.shading_level = sb6::program::get_uniform_location(
            self.render_program, "shading_level").unwrap();

        let ssao_shaders = [
            sb6::shader::load("media/shaders/ssao/ssao.vs.glsl",
                gl::VERTEX_SHADER).unwrap(),
            sb6::shader::load("media/shaders/ssao/ssao.fs.glsl",
                gl::FRAGMENT_SHADER).unwrap(),
            ];

        self.ssao_program = sb6::program::link_from_shaders(
            &ssao_shaders).unwrap();

        self.ssao.ssao_radius = sb6::program::get_uniform_location(
            self.ssao_program, "ssao_radius").unwrap();
        self.ssao.ssao_level = sb6::program::get_uniform_location(
            self.ssao_program, "ssao_level").unwrap();
        self.ssao.object_level = sb6::program::get_uniform_location(
            self.ssao_program, "object_level").unwrap();
        self.ssao.randomize_points = sb6::program::get_uniform_location(
            self.ssao_program, "randomize_points").unwrap();
        self.ssao.point_count = sb6::program::get_uniform_location(
            self.ssao_program, "point_count").unwrap();
    }
}

impl sb6::App for MyApp {
    fn get_app_info(&self) -> &sb6::AppInfo { &self.info }
    fn startup(&mut self) {
        self.load_shaders();

        unsafe {
            gl::GenFramebuffers(1, &mut self.render_fbo);
            gl::BindFramebuffer(gl::FRAMEBUFFER, self.render_fbo);
            gl::GenTextures(3, mem::transmute(self.fbo_textures.as_ptr()));

            gl::BindTexture(gl::TEXTURE_2D, self.fbo_textures[0]);
            gl::TexStorage2D(gl::TEXTURE_2D, 1, gl::RGB16F, 2048, 2048);
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MIN_FILTER, gl::NEAREST as GLint);
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MAG_FILTER, gl::NEAREST as GLint);

            gl::BindTexture(gl::TEXTURE_2D, self.fbo_textures[1]);
            gl::TexStorage2D(gl::TEXTURE_2D, 1, gl::RGBA32F, 2048, 2048);
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MIN_FILTER, gl::NEAREST as GLint);
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MAG_FILTER, gl::NEAREST as GLint);

            gl::BindTexture(gl::TEXTURE_2D, self.fbo_textures[2]);
            gl::TexStorage2D(gl::TEXTURE_2D, 1, gl::DEPTH_COMPONENT32F, 2048, 2048);
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MIN_FILTER, gl::NEAREST as GLint);
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MAG_FILTER, gl::NEAREST as GLint);
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_S, gl::CLAMP_TO_EDGE as GLint);
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_T, gl::CLAMP_TO_EDGE as GLint);

            gl::FramebufferTexture(gl::FRAMEBUFFER, gl::COLOR_ATTACHMENT0, self.fbo_textures[0], 0);
            gl::FramebufferTexture(gl::FRAMEBUFFER, gl::COLOR_ATTACHMENT1, self.fbo_textures[1], 0);
            gl::FramebufferTexture(gl::FRAMEBUFFER, gl::DEPTH_ATTACHMENT, self.fbo_textures[2], 0);

            const DRAW_BUFFERS: [GLint, ..2] = [ gl::COLOR_ATTACHMENT0 as GLint,
                gl::COLOR_ATTACHMENT1 as GLint ];

            gl::DrawBuffers(2, mem::transmute(DRAW_BUFFERS.as_ptr()));

            gl::BindFramebuffer(gl::FRAMEBUFFER, 0);

            gl::GenVertexArrays(1, &mut self.quad_vao);
            gl::BindVertexArray(self.quad_vao);
        }

        self.object.load("media/objects/dragon.sbm").unwrap();
        self.cube.load("media/objects/cube.sbm").unwrap();

        unsafe {
            gl::Enable(gl::DEPTH_TEST);
            gl::Enable(gl::CULL_FACE);
        }

        let mut rng = rand::weak_rng();
        let mut point_data = SamplePoints {
            point: [vmath::Vec4::zero(), ..256],
            random_vectors: [vmath::Vec4::zero(), ..256]
        };

        for i in range(0, 256) {
            loop
            {
                point_data.point[i] = vmath::Vec4::new(
                    rng.gen::<f32>() * 2.0 - 1.0,
                    rng.gen::<f32>() * 2.0 - 1.0,
                    rng.gen::<f32>(), //  * 2.0 - 1.0;
                    0.0);
                if point_data.point[i].length() <= 1.0 {
                    break;
                }
            }
            point_data.point[i].normalize();
        }
        for i in range(0, 256) {
            point_data.random_vectors[i] = vmath::Vec4::new(
                rng.gen::<f32>(), rng.gen::<f32>(),
                rng.gen::<f32>(), rng.gen::<f32>());
        }

        unsafe {
            gl::GenBuffers(1, &mut self.points_buffer);
            gl::BindBuffer(gl::UNIFORM_BUFFER, self.points_buffer);
            gl::BufferData(gl::UNIFORM_BUFFER,
                mem::size_of::<SamplePoints>() as GLsizeiptr,
                mem::transmute(&point_data), gl::STATIC_DRAW);
        }
    }

    fn shutdown(&mut self) {
        unsafe {
            gl::DeleteProgram(self.render_program);
            gl::DeleteTextures(3, self.fbo_textures.as_ptr());
        }
        self.object.free();
        self.cube.free();
        self.render_program = 0;
        self.ssao_program = 0;
        self.fbo_textures = [0, ..3];
        self.render = RenderUniforms::new();
        self.ssao = SSAOUniforms::new();
    }

    fn update(&mut self, current_time: f64) {
        if !self.paused {
            self.total_time += current_time - self.last_time;
        }
        self.last_time = current_time;
    }

    fn render(&self, _: f64) {
        const BLACK: [GLfloat, ..4] = [ 0.0, 0.0, 0.0, 0.0 ];
        const ONE: GLfloat = 1.0;

        let f = self.total_time as f32;

        let lookat_matrix = vmath::Mat4::lookat(
            &vmath::Vec3::new(0.0, 3.0, 15.0),
            &vmath::Vec3::new(0.0, 0.0, 0.0),
            &vmath::Vec3::new(0.0, 1.0, 0.0));
        let aspect = self.info.window_width as f32 /
            self.info.window_height as f32;
        let proj_matrix = vmath::Mat4::perspective(50.0, aspect, 0.1, 1000.0);

        let shading_level =
            if self.show_shading { if self.show_ao { 0.7 } else { 1.0 } } 
            else { 0.0 };

        let ssao_level =
            if self.show_ao { if self.show_shading { 0.3 } else { 1.0 } }
            else { 0.0 };

        unsafe {
            gl::Viewport(0, 0, self.info.window_width as GLint,
                         self.info.window_height as GLint);

            gl::BindFramebuffer(gl::FRAMEBUFFER, self.render_fbo);
            gl::Enable(gl::DEPTH_TEST);

            gl::ClearBufferfv(gl::COLOR, 0, BLACK.as_ptr());
            gl::ClearBufferfv(gl::COLOR, 1, BLACK.as_ptr());
            gl::ClearBufferfv(gl::DEPTH, 0, &ONE);

            gl::BindBufferBase(gl::UNIFORM_BUFFER, 0, self.points_buffer);

            gl::UseProgram(self.render_program);

            gl::UniformMatrix4fv(self.render.proj_matrix, 1, gl::FALSE,
                                 proj_matrix.as_ptr());

            let mv_matrix = vmath::Mat4::translate(0.0, -5.0, 0.0) *
                vmath::Mat4::rotate(f * 5.0, 0.0, 1.0, 0.0) *
                vmath::Mat4::identity();
            gl::UniformMatrix4fv(self.render.mv_matrix, 1, gl::FALSE,
                                 (lookat_matrix * mv_matrix).as_ptr());

            gl::Uniform1f(self.render.shading_level, shading_level);

            self.object.render();
        }

        unsafe {
            let mv_matrix = vmath::Mat4::translate(0.0, -4.5, 0.0) *
                vmath::Mat4::rotate(f * 5.0, 0.0, 1.0, 0.0) *
                vmath::Mat4::scale(4000.0, 0.1, 4000.0) *
                vmath::Mat4::identity();
            gl::UniformMatrix4fv(self.render.mv_matrix, 1, gl::FALSE,
                (lookat_matrix * mv_matrix).as_ptr());

            self.cube.render();
        }

        unsafe {
            gl::BindFramebuffer(gl::FRAMEBUFFER, 0);

            gl::UseProgram(self.ssao_program);

            gl::Uniform1f(self.ssao.ssao_radius, self.ssao_radius *
                self.info.window_width as f32 / 1000.0);

            gl::Uniform1f(self.ssao.ssao_level, ssao_level);
            // let weight_by_angle = if self.weight_by_angle { 1 } else { 0 };
            // gl::Uniform1i(self.ssao.weight_by_angle, weight_by_angle);
            let randomize_points = if self.randomize_points { 1 } else { 0 };
            gl::Uniform1i(self.ssao.randomize_points, randomize_points);
            gl::Uniform1ui(self.ssao.point_count, self.point_count);

            gl::ActiveTexture(gl::TEXTURE0);
            gl::BindTexture(gl::TEXTURE_2D, self.fbo_textures[0]);
            gl::ActiveTexture(gl::TEXTURE1);
            gl::BindTexture(gl::TEXTURE_2D, self.fbo_textures[1]);

            gl::Disable(gl::DEPTH_TEST);
            gl::BindVertexArray(self.quad_vao);
            gl::DrawArrays(gl::TRIANGLE_STRIP, 0, 4);
        }
    }

    fn on_key(&mut self, key: sb6::Key, action: sb6::Action)
    {
        if action == sb6::Action::Release {
            match key {
                sb6::Key::R => self.randomize_points = !self.randomize_points,
                sb6::Key::S => self.point_count += 1,
                sb6::Key::X => self.point_count -= 1,
                sb6::Key::Q => self.show_shading = !self.show_shading,
                sb6::Key::W => self.show_ao = !self.show_ao,
                sb6::Key::A => self.ssao_radius += 0.01,
                sb6::Key::Z => self.ssao_radius -= 0.01,
                sb6::Key::P => self.paused = !self.paused,
                sb6::Key::L => self.load_shaders(),
                _ => ()
            };
        }
    }
}

fn main() {
    let mut init = sb6::AppInfo::default();
    init.title = "OpenGL SuperBible - SSAO";
    let mut app = MyApp::new(init);
    sb6::run(&mut app);
}

