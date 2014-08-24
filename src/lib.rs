#![crate_type = "lib"]
#![crate_name = "sb6"]
#![comment = "OpenGL Super Bible Application Framework"]

#![feature(globs)]
#![feature(macro_rules)]

extern crate gl;
extern crate glfw;

pub use app::AppInfo;
pub use app::App;
pub use app::check_compile_status;
pub use app::check_link_status;
pub use app::run;

pub use object::Object;
pub use object::ObjectError;

mod app;
mod object;
