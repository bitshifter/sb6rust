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

use std::f32;
use std::fmt;
use std::ops::Add;
use std::ops::Mul;
use std::ops::Sub;

#[inline]
fn deg_to_rad(a: f32) -> f32 {
    f32::consts::PI * 2.0 * a / 360.0
}

#[derive(Clone, Copy, Debug)]
pub struct Vec3 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

#[allow(dead_code)]
pub fn vec3(x: f32, y: f32, z: f32) -> Vec3 {
    Vec3 { x, y, z }
}

#[allow(dead_code)]
impl Vec3 {
    pub fn zero() -> Vec3 {
        Vec3 {
            x: 0.0,
            y: 0.0,
            z: 0.0,
        }
    }
    pub fn dot(&self, rhs: &Vec3) -> f32 {
        (self.x * rhs.x) + (self.y * rhs.y) + (self.z * rhs.z)
    }
    pub fn cross(&self, rhs: &Vec3) -> Vec3 {
        Vec3 {
            x: self.y * rhs.z - rhs.y * self.z,
            y: self.z * rhs.x - rhs.z * self.x,
            z: self.x * rhs.y - rhs.x * self.y,
        }
    }
    pub fn length(&self) -> f32 {
        self.dot(self).sqrt()
    }
    pub fn normalize(&self) -> Vec3 {
        let inv_length = 1.0 / self.dot(self).sqrt();
        *self * inv_length
    }
}

impl fmt::Display for Vec3 {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "[{}, {}, {}]", self.x, self.y, self.z)
    }
}

impl Mul<Vec3> for Vec3 {
    type Output = Vec3;
    fn mul(self, rhs: Vec3) -> Vec3 {
        Vec3 {
            x: self.x * rhs.x,
            y: self.y * rhs.y,
            z: self.z * rhs.z,
        }
    }
}

impl Mul<f32> for Vec3 {
    type Output = Vec3;
    fn mul(self, rhs: f32) -> Vec3 {
        Vec3 {
            x: self.x * rhs,
            y: self.y * rhs,
            z: self.z * rhs,
        }
    }
}

impl Add for Vec3 {
    type Output = Vec3;
    fn add(self, rhs: Vec3) -> Vec3 {
        Vec3 {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z,
        }
    }
}

impl Sub for Vec3 {
    type Output = Vec3;
    fn sub(self, rhs: Vec3) -> Vec3 {
        Vec3 {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
            z: self.z - rhs.z,
        }
    }
}

#[derive(Clone, Copy, Debug)]
pub struct Vec4 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
    pub w: f32,
}

pub fn vec4(x: f32, y: f32, z: f32, w: f32) -> Vec4 {
    Vec4 { x, y, z, w }
}

#[allow(dead_code)]
impl Vec4 {
    pub fn zero() -> Vec4 {
        Vec4 {
            x: 0.0,
            y: 0.0,
            z: 0.0,
            w: 0.0,
        }
    }
    pub fn dot(&self, rhs: &Vec4) -> f32 {
        (self.x * rhs.x) + (self.y * rhs.y) + (self.z * rhs.z) + (self.w * rhs.w)
    }
    pub fn length(&self) -> f32 {
        self.dot(self).sqrt()
    }
    pub fn normalize(&self) -> Vec4 {
        let inv_length = 1.0 / self.dot(self).sqrt();
        *self * inv_length
    }
}

impl Mul<Vec4> for Vec4 {
    type Output = Vec4;
    fn mul(self, rhs: Vec4) -> Vec4 {
        Vec4 {
            x: self.x * rhs.x,
            y: self.y * rhs.y,
            z: self.z * rhs.z,
            w: self.w * rhs.w,
        }
    }
}

impl Mul<f32> for Vec4 {
    type Output = Vec4;
    fn mul(self, rhs: f32) -> Vec4 {
        Vec4 {
            x: self.x * rhs,
            y: self.y * rhs,
            z: self.z * rhs,
            w: self.w * rhs,
        }
    }
}

impl Add for Vec4 {
    type Output = Vec4;
    fn add(self, rhs: Vec4) -> Vec4 {
        Vec4 {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z,
            w: self.w + rhs.w,
        }
    }
}

impl Sub for Vec4 {
    type Output = Vec4;
    fn sub(self, rhs: Vec4) -> Vec4 {
        Vec4 {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
            z: self.z - rhs.z,
            w: self.w - rhs.w,
        }
    }
}

impl fmt::Display for Vec4 {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "[{}, {}, {}, {}]", self.x, self.y, self.z, self.w)
    }
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
    let s = f.cross(&up_n);
    let u = s.cross(&f);
    Mat4 {
        col0: vec4(s.x, u.x, -f.x, 0.0),
        col1: vec4(s.y, u.y, -f.y, 0.0),
        col2: vec4(s.z, u.z, -f.z, 0.0),
        col3: vec4(-s.dot(&eye), -u.dot(&eye), f.dot(&eye), 1.0),
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
    pub fn as_ptr(&self) -> *const f32 {
        &self.col0.x as *const f32
    }
    pub fn zero() -> Mat4 {
        Mat4 {
            col0: Vec4::zero(),
            col1: Vec4::zero(),
            col2: Vec4::zero(),
            col3: Vec4::zero(),
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
            col0: (a0 * b0.x) + (a1 * b0.y) + (a2 * b0.z) + (a3 * b0.w),
            col1: (a0 * b1.x) + (a1 * b1.y) + (a2 * b1.z) + (a3 * b1.w),
            col2: (a0 * b2.x) + (a1 * b2.y) + (a2 * b2.z) + (a3 * b2.w),
            col3: (a0 * b3.x) + (a1 * b3.y) + (a2 * b3.z) + (a3 * b3.w),
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
