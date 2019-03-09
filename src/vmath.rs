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

pub use glam::{vec3, vec4, Vec3, Vec4};
use std::f32;
use std::fmt;
use std::mem;
use std::ops::Mul;

#[inline]
fn deg_to_rad(a: f32) -> f32 {
    f32::consts::PI * 2.0 * a / 360.0
}

#[derive(Clone, Copy, Debug)]
pub struct Mat4 {
    pub col0: Vec4,
    pub col1: Vec4,
    pub col2: Vec4,
    pub col3: Vec4,
}

#[allow(dead_code)]
pub fn mat4(col0: Vec4, col1: Vec4, col2: Vec4, col3: Vec4) -> Mat4 {
    Mat4 {
        col0,
        col1,
        col2,
        col3,
    }
}

#[allow(dead_code)]
pub fn perspective(fovy: f32, aspect: f32, near: f32, far: f32) -> Mat4 {
    let q = 1.0 / deg_to_rad(0.5 * fovy).tan();
    let a = q / aspect;
    let b = (near + far) / (near - far);
    let c = (2.0 * near * far) / (near - far);

    Mat4 {
        col0: vec4(a, 0.0, 0.0, 0.0),
        col1: vec4(0.0, q, 0.0, 0.0),
        col2: vec4(0.0, 0.0, b, -1.0),
        col3: vec4(0.0, 0.0, c, 0.0),
    }
}

#[allow(dead_code)]
pub fn translate(x: f32, y: f32, z: f32) -> Mat4 {
    Mat4 {
        col0: vec4(1.0, 0.0, 0.0, 0.0),
        col1: vec4(0.0, 1.0, 0.0, 0.0),
        col2: vec4(0.0, 0.0, 1.0, 0.0),
        col3: vec4(x, y, z, 1.0),
    }
}

#[allow(dead_code)]
pub fn look_at(eye: Vec3, center: Vec3, up: Vec3) -> Mat4 {
    let f = (center - eye).normalize();
    let up_n = up.normalize();
    let s = f.cross(up_n);
    let u = s.cross(f);
    Mat4 {
        col0: vec4(s.get_x(), u.get_x(), -f.get_x(), 0.0),
        col1: vec4(s.get_y(), u.get_y(), -f.get_y(), 0.0),
        col2: vec4(s.get_z(), u.get_z(), -f.get_z(), 0.0),
        col3: vec4(-s.dot(eye), -u.dot(eye), f.dot(eye), 1.0),
    }
}

#[allow(dead_code)]
pub fn rotate(angle: f32, x: f32, y: f32, z: f32) -> Mat4 {
    let x2 = x * x;
    let y2 = y * y;
    let z2 = z * z;
    let rads = deg_to_rad(angle);
    let (sin, cos) = rads.sin_cos();
    let omc = 1.0 - cos;
    Mat4 {
        col0: vec4(
            x2 * omc + cos,
            y * x * omc + z * sin,
            x * z * omc - y * sin,
            0.0,
        ),
        col1: vec4(
            x * y * omc - z * sin,
            y2 * omc + cos,
            y * z * omc + x * sin,
            0.0,
        ),
        col2: vec4(
            x * z * omc + y * sin,
            y * z * omc - x * sin,
            z2 * omc + cos,
            0.0,
        ),
        col3: vec4(0.0, 0.0, 0.0, 1.0),
    }
}

#[allow(dead_code)]
pub fn scale(x: f32, y: f32, z: f32) -> Mat4 {
    Mat4 {
        col0: vec4(x, 0.0, 0.0, 0.0),
        col1: vec4(0.0, y, 0.0, 0.0),
        col2: vec4(0.0, 0.0, z, 0.0),
        col3: vec4(0.0, 0.0, 0.0, 1.0),
    }
}

#[allow(dead_code)]
pub fn identity() -> Mat4 {
    Mat4 {
        col0: vec4(1.0, 0.0, 0.0, 0.0),
        col1: vec4(0.0, 1.0, 0.0, 0.0),
        col2: vec4(0.0, 0.0, 1.0, 0.0),
        col3: vec4(0.0, 0.0, 0.0, 1.0),
    }
}

#[allow(dead_code)]
impl Mat4 {
    pub fn zero() -> Mat4 {
        Mat4 {
            col0: Vec4::zero(),
            col1: Vec4::zero(),
            col2: Vec4::zero(),
            col3: Vec4::zero(),
        }
    }

    #[inline]
    pub unsafe fn as_ptr(&self) -> *const f32 {
        std::mem::transmute(&self.col0)
    }

    pub fn store_to_slice(&self, slice: &mut [f32]) {
        self.col0.store_to_slice(&mut slice[0..4]);
        self.col1.store_to_slice(&mut slice[4..8]);
        self.col2.store_to_slice(&mut slice[8..12]);
        self.col3.store_to_slice(&mut slice[12..16]);
    }
}

impl From<Mat4> for [f32; 16] {
    fn from(m: Mat4) -> Self {
        unsafe {
            let mut out: [f32; 16] = mem::uninitialized();
            m.col0.store_to_slice(&mut out[0..4]);
            m.col1.store_to_slice(&mut out[4..8]);
            m.col2.store_to_slice(&mut out[8..12]);
            m.col3.store_to_slice(&mut out[12..16]);
            out
        }
    }
}

impl Mul for Mat4 {
    type Output = Mat4;
    fn mul(self, rhs: Mat4) -> Mat4 {
        let a0 = self.col0;
        let a1 = self.col1;
        let a2 = self.col2;
        let a3 = self.col3;

        let b0 = rhs.col0;
        let b1 = rhs.col1;
        let b2 = rhs.col2;
        let b3 = rhs.col3;

        Mat4 {
            col0: (a0 * b0.get_x()) + (a1 * b0.get_y()) + (a2 * b0.get_z()) + (a3 * b0.get_w()),
            col1: (a0 * b1.get_x()) + (a1 * b1.get_y()) + (a2 * b1.get_z()) + (a3 * b1.get_w()),
            col2: (a0 * b2.get_x()) + (a1 * b2.get_y()) + (a2 * b2.get_z()) + (a3 * b2.get_w()),
            col3: (a0 * b3.get_x()) + (a1 * b3.get_y()) + (a2 * b3.get_z()) + (a3 * b3.get_w()),
        }
    }
}

impl fmt::Display for Mat4 {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "[{}, {}, {}, {}]",
            self.col0, self.col1, self.col2, self.col3
        )
    }
}
