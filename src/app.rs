extern crate gl;
extern crate glfw;

use gl::types::*;
use glfw::Context;
use std::ptr;
use std::str;

pub struct AppInfo {
    pub title: &'static str,
    pub windowWidth: u32,
    pub windowHeight: u32,
    pub majorVersion: u32,
    pub minorVersion: u32,
    pub samples: uint,
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
    fn version() -> (u32, u32) { (4, 4) }
    pub fn default() -> AppInfo {
        let (major, minor) = AppInfo::version();
        AppInfo {
        title: "SuperBible6 Example",
        windowWidth: 800,
        windowHeight: 600,
        majorVersion: major,
        minorVersion: minor,
        samples: 0,
        fullscreen: false,
        vsync: false,
        cursor: true,
        stereo: false,
        debug: false
        }
    }
}

pub fn check_compile_status(shader: GLuint) {
        unsafe {
            // Get the compile status
            let mut status = gl::FALSE as GLint;
            gl::GetShaderiv(shader, gl::COMPILE_STATUS, &mut status);

            // Fail on error
            if status != (gl::TRUE as GLint) {
                let mut len = 0;
                gl::GetShaderiv(shader, gl::INFO_LOG_LENGTH, &mut len);
                // subtract 1 to skip the trailing null character
                let mut buf = Vec::from_elem(len as uint - 1, 0u8);
                gl::GetShaderInfoLog(shader, len, ptr::mut_null(),
                    buf.as_mut_ptr() as *mut GLchar);
                fail!("{}", str::from_utf8(buf.as_slice()).expect(
                        "ShaderInfoLog not valid utf8"));
            }
        }
}

pub fn check_link_status(program: GLuint) {
    unsafe {
        // Get the link status
        let mut status = gl::FALSE as GLint;
        gl::GetProgramiv(program, gl::LINK_STATUS, &mut status);

        // Fail on error
        if status != (gl::TRUE as GLint) {
            let mut len: GLint = 0;
            gl::GetProgramiv(program, gl::INFO_LOG_LENGTH, &mut len);
            // subtract 1 to skip the trailing null character
            let mut buf = Vec::from_elem(len as uint - 1, 0u8);
            gl::GetProgramInfoLog(program, len, ptr::mut_null(),
                buf.as_mut_ptr() as *mut GLchar);
            fail!("{}", str::from_utf8(buf.as_slice()).expect(
                    "ProgramInfoLog not valid utf8"));
        }
    }
}

fn handle_window_event(window: &glfw::Window, event: glfw::WindowEvent) {
    match event {
        glfw::KeyEvent(glfw::KeyEscape, _, glfw::Press, _) => {
            window.set_should_close(true)
        }
        _ => {}
    }
}

pub trait App
{
    fn get_app_info(&self) -> &AppInfo;
    fn startup(&mut self);
    fn render(&self, time: f64);
    fn shutdown(&mut self);
    fn on_resize(&mut self, _: int, _: int) {}
}

pub fn run<T: App>(app: &mut T) {
    let glfw = glfw::init(glfw::FAIL_ON_ERRORS).unwrap();
    let (window, events) = {
        let info = app.get_app_info();
        glfw.window_hint(glfw::ContextVersion(
                info.majorVersion, info.minorVersion));
        glfw.window_hint(glfw::OpenglProfile(glfw::OpenGlCoreProfile));
        glfw.window_hint(glfw::OpenglForwardCompat(true));
        glfw.create_window(
            info.windowWidth, info.windowHeight, info.title.as_slice(),
            glfw::Windowed).expect("Failed to create GLFW window.")
    };

    window.set_key_polling(true);
    window.make_current();

    // Load the OpenGL function pointers
    gl::load_with(|s| glfw.get_proc_address(s));

    app.startup();

    while !window.should_close() {
        app.render(glfw.get_time());

        window.swap_buffers();

        glfw.poll_events();
        for (_, event) in glfw::flush_messages(&events) {
            handle_window_event(&window, event);
        }
    }

    app.shutdown();
}
