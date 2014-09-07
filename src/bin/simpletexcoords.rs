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

extern crate gl;
extern crate glfw;
extern crate native;
extern crate sb6;

use gl::types::*;
use std::mem;

mod vmath;

struct MyApp {
    info: sb6::AppInfo,
    render_prog: GLuint,
    tex_object: [GLuint, ..2],
    tex_index: GLuint,
    mv_matrix: GLint,
    proj_matrix: GLint,
    object: sb6::object::Object
}

impl MyApp {
    fn new(init: sb6::AppInfo) -> MyApp {
        MyApp {
            info: init,
            render_prog: 0,
            tex_object: [0, ..2],
            tex_index: 0,
            mv_matrix: -1,
            proj_matrix: -1,
            object: sb6::object::Object::new()
        }
    }

    fn load_shaders(&mut self) {
        if self.render_prog != 0 {
            gl::DeleteProgram(self.render_prog);
        }

        let vs = sb6::shader::load(
            "media/shaders/simpletexcoords/render.vs.glsl",
            gl::VERTEX_SHADER).unwrap();
        let fs = sb6::shader::load(
            "media/shaders/simpletexcoords/render.fs.glsl",
            gl::FRAGMENT_SHADER).unwrap();

        self.render_prog = gl::CreateProgram();
        gl::AttachShader(self.render_prog, vs);
        gl::AttachShader(self.render_prog, fs);
        gl::LinkProgram(self.render_prog);
        sb6::program::assert_link_status(self.render_prog);

        gl::DeleteShader(vs);
        gl::DeleteShader(fs);

        self.mv_matrix = "mv_matrix".with_c_str(
            |ptr| unsafe { gl::GetUniformLocation(self.render_prog, ptr) });
        self.proj_matrix = "proj_matrix".with_c_str(
            |ptr| unsafe { gl::GetUniformLocation(self.render_prog, ptr) });
    }
}

impl sb6::App for MyApp {
    fn get_app_info(&self) -> &sb6::AppInfo { &self.info }
    fn startup(&mut self) {
        // generate a 16 x 16 checker texture
        static tex_dim: uint = 16;
        let mut tex_data : [u32, ..(tex_dim * tex_dim)] =
            [0, ..(tex_dim * tex_dim)];
        for i in range(0, tex_data.len())
        {
            let col = i % tex_dim;
            let row = i / tex_dim;
            if row % 2 == 0 {
                if col % 2 == 0 {
                    tex_data[i] = 0xffffffff;
                }
            }
            else {
                if col % 2 == 1 {
                    tex_data[i] = 0xffffffff;
                }
            }
        }

        unsafe {
            gl::GenTextures(1, &mut self.tex_object[0]);
            gl::BindTexture(gl::TEXTURE_2D, self.tex_object[0]);
            gl::TexStorage2D(gl::TEXTURE_2D, 1, gl::RGB8, 16, 16);
            gl::TexSubImage2D(gl::TEXTURE_2D, 0, 0, 0, 16, 16, gl::RGBA,
                gl::UNSIGNED_BYTE, mem::transmute(tex_data.as_ptr()));
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MIN_FILTER,
                gl::NEAREST as i32);
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MAG_FILTER,
                gl::NEAREST as i32);
        }

        self.tex_object[1] = sb6::ktx::load(
            "media/textures/pattern1.ktx").unwrap();

        self.object.load("media/objects/torus_nrms_tc.sbm").unwrap();

        self.load_shaders();

        gl::Enable(gl::DEPTH_TEST);
        gl::DepthFunc(gl::LEQUAL);
    }

    fn shutdown(&mut self) {
        gl::DeleteProgram(self.render_prog);
        unsafe { gl::DeleteTextures(2, self.tex_object.as_ptr()) };
        self.object.free();
        self.render_prog = 0;
        self.tex_object = [0, ..2];
        self.mv_matrix = -1;
        self.proj_matrix = -1;
    }

    fn render(&self, current_time: f64) {
        let gray = [ 0.2, 0.2, 0.2, 1.0 ];
        let ones = [ 1.0 ];

        unsafe {
            gl::ClearBufferfv(gl::COLOR, 0, gray.as_ptr());
            gl::ClearBufferfv(gl::DEPTH, 0, ones.as_ptr());
        }

        gl::Viewport(0, 0, self.info.window_width as i32,
            self.info.window_height as i32);

        gl::BindTexture(gl::TEXTURE_2D, self.tex_object[self.tex_index as uint]);

        gl::UseProgram(self.render_prog);

        let aspect = self.info.window_width as f32 /
            self.info.window_height as f32;
        let proj_matrix = vmath::Mat4::perspective(60.0, aspect, 0.1, 1000.0);
        let mv_matrix = vmath::Mat4::translate(0.0, 0.0, -3.0) *
            vmath::Mat4::rotate(current_time as f32 * 19.3, 0.0, 1.0, 0.0) *
            vmath::Mat4::rotate(current_time as f32 * 21.1, 0.0, 0.0, 1.0);

        unsafe {
            gl::UniformMatrix4fv(self.mv_matrix, 1, gl::FALSE, mv_matrix.as_ptr());
            gl::UniformMatrix4fv(self.proj_matrix, 1, gl::FALSE, proj_matrix.as_ptr());
        }

        self.object.render();
    }

    fn on_key(&mut self, key: glfw::Key, action: glfw::Action)
    {
        if action == glfw::Release {
            match key {
                glfw::KeyR => self.load_shaders(),
                glfw::KeyT => self.tex_index = (self.tex_index + 1) % 2,
                _ => ()
            };
        }
    }
}

fn main() {
    let mut init = sb6::AppInfo::default();
    init.title = "OpenGL SuperBible - Texture Coordinates";
    let mut app = MyApp::new(init);
    sb6::run(&mut app);
}

#[start]
fn start(argc: int, argv: *const *const u8) -> int {
    native::start(argc, argv, main)
}

