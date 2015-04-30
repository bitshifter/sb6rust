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
extern crate glfw;

use glfw::Context;

// Re-export some glfw enums required for event handling
pub use glfw::Key;
pub use glfw::Action;

#[derive(Clone)]
pub struct AppInfo {
    pub title: &'static str,
    pub window_width: u32,
    pub window_height: u32,
    pub major_version: u32,
    pub minor_version: u32,
    pub samples: usize,
    pub fullscreen: bool,
    pub vsync: bool,
    pub cursor: bool,
    pub stereo: bool,
    pub debug: bool
}

impl AppInfo {
    #[cfg(use_gl_3_3)]
    fn version() -> (u32, u32) { (3, 3) }
    #[cfg(not(use_gl_3_3))]
    fn version() -> (u32, u32) { (4, 3) }
    pub fn default() -> AppInfo {
        let (major, minor) = AppInfo::version();
        AppInfo {
        title: "SuperBible6 Example",
        window_width: 800,
        window_height: 600,
        major_version: major,
        minor_version: minor,
        samples: 0,
        fullscreen: false,
        vsync: false,
        cursor: true,
        stereo: false,
        debug: false
        }
    }
}

pub trait App
{
    fn get_app_info(&self) -> &AppInfo;
    fn startup(&mut self) {}
    fn update(&mut self, _: f64) {}
    fn render(&self, _: f64) {}
    fn shutdown(&mut self) {}
    fn on_resize(&mut self, _: isize, _: isize) {}
    fn on_key(&mut self, _: Key, _: Action) {}
}

fn handle_window_event<T: App>(app: &mut T, window: &mut glfw::Window,
                               event: glfw::WindowEvent) {
    match event {
        glfw::WindowEvent::Key(glfw::Key::Escape, _, glfw::Action::Press, _) => {
            window.set_should_close(true)
        }
        glfw::WindowEvent::Key(key, _, action, _) => {
            app.on_key(key, action)
        },
        glfw::WindowEvent::Size(w, h) => {
            app.on_resize(w as isize, h as isize)
        }
        _ => ()
    }
}

pub fn run<T: App>(app: &mut T) {
    let mut glfw = glfw::init(glfw::FAIL_ON_ERRORS).unwrap();
    let (mut window, events) = {
        let info = app.get_app_info();
        glfw.window_hint(glfw::WindowHint::ContextVersion(
                info.major_version, info.minor_version));
        glfw.window_hint(glfw::WindowHint::OpenGlProfile(glfw::OpenGlProfileHint::Core));
        glfw.window_hint(glfw::WindowHint::OpenGlForwardCompat(true));
        glfw.create_window(
            info.window_width, info.window_height, &info.title,
            glfw::WindowMode::Windowed).expect("Failed to create GLFW window.")
    };

    window.set_key_polling(true);
    window.set_size_polling(true);
    window.make_current();

    // Load the OpenGL function pointers
    gl::load_with(|s| window.get_proc_address(s));

    app.startup();

    while !window.should_close() {
        let time = glfw.get_time();
        app.update(time);
        app.render(time);

        window.swap_buffers();

        glfw.poll_events();
        for (_, event) in glfw::flush_messages(&events) {
            handle_window_event::<T>(app, &mut window, event);
        }
    }

    app.shutdown();
}
