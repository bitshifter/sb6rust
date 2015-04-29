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

struct MyApp {
    info: sb6::AppInfo,
}

impl MyApp {
    fn new(init: sb6::AppInfo) -> MyApp {
        MyApp { info: init }
    }
}

impl sb6::App for MyApp {
    fn get_app_info(&self) -> &sb6::AppInfo { &self.info }

    fn render(&self, time: f64) {
        const RED: [GLfloat; 4] = [ 1.0, 0.0, 0.0, 1.0 ];
        unsafe {
            gl::ClearBufferfv(gl::COLOR, 0, RED.as_ptr());
        }
    }
}

fn main() {
    let mut init = sb6::AppInfo::default();
    init.title = "OpenGL SuperBible - Single Triangle";
    let mut app = MyApp::new(init);
    sb6::run(&mut app);
}

