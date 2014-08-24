#![feature(globs)]

extern crate gl;
extern crate native;
extern crate sb6;

use gl::types::*;
use std::ptr;

static VS_SRC: &'static str = "\
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

static TCS_SRC: &'static str = "\
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

static TES_SRC: &'static str = "\
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

static FS_SRC: &'static str = "\
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
        let fs = gl::CreateShader(gl::FRAGMENT_SHADER);
        unsafe {
            VS_SRC.with_c_str(
                |ptr| gl::ShaderSource(vs, 1, &ptr, ptr::null()));
            gl::CompileShader(vs);
            sb6::check_compile_status(vs);

            TCS_SRC.with_c_str(
                |ptr| gl::ShaderSource(tcs, 1, &ptr, ptr::null()));
            gl::CompileShader(tcs);
            sb6::check_compile_status(tcs);

            TES_SRC.with_c_str(
                |ptr| gl::ShaderSource(tes, 1, &ptr, ptr::null()));
            gl::CompileShader(tes);
            sb6::check_compile_status(tes);

            FS_SRC.with_c_str(
                |ptr| gl::ShaderSource(fs, 1, &ptr, ptr::null()));
            gl::CompileShader(fs);
            sb6::check_compile_status(fs);
        }

        gl::AttachShader(self.program, vs);
        gl::AttachShader(self.program, tcs);
        gl::AttachShader(self.program, tes);
        gl::AttachShader(self.program, fs);

        gl::LinkProgram(self.program);
        sb6::check_link_status(self.program);

        gl::DeleteShader(vs);
        gl::DeleteShader(tcs);
        gl::DeleteShader(tes);
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
        static green: [GLfloat, ..4] = [ 0.0, 0.25, 0.0, 1.0 ];
        unsafe {
            gl::ClearBufferfv(gl::COLOR, 0, green.as_ptr());
        }

        gl::UseProgram(self.program);
        gl::DrawArrays(gl::PATCHES, 0, 3);
    }
}

fn main() {
    let mut init = sb6::AppInfo::default();
    init.title = "OpenGL SuperBible - Tessellated Triangle";
    let mut app = MyApp::new(init);
    sb6::run(&mut app);
}

#[start]
fn start(argc: int, argv: *const *const u8) -> int {
    native::start(argc, argv, main)
}

