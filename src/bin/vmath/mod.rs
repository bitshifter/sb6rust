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
use std::fmt;
use std::num::Float;
use std::num::FloatMath;

#[allow(dead_code)]
pub struct Vec3 {
    v: [f32, ..3]
}

#[allow(dead_code)]
impl Vec3 {
    pub fn new(x: f32, y: f32, z: f32) -> Vec3 {
        Vec3 { v: [x, y, z] }
    }
    pub fn zero() -> Vec3 {
        Vec3 { v: [0.0, 0.0, 0.0] }
    }
    pub fn identity() -> Vec3 {
        Vec3 { v: [1.0, 1.0, 1.0] }
    }
    pub fn scale(&self, s: f32) -> Vec3 {
        Vec3 { v: [
            self.v[0] * s,
            self.v[1] * s,
            self.v[2] * s]
        }
    }
    pub fn dot(&self, rhs: &Vec3) -> f32 {
        (self.v[0] * rhs.v[0]) + (self.v[1] * rhs.v[1]) +
            (self.v[2] * rhs.v[2])
    }
    pub fn cross(&self, rhs: &Vec3) -> Vec3 {
        Vec3 { v: [
            self.v[1] * rhs.v[2] - rhs.v[1] * self.v[2],
            self.v[2] * rhs.v[0] - rhs.v[2] * self.v[0],
            self.v[0] * rhs.v[1] - rhs.v[0] * self.v[1]]
        }
    }
    pub fn length(&self) -> f32 {
        self.dot(self).sqrt()
    }
    pub fn normalize(&self) -> Vec3 {
        let inv_length = self.dot(self).rsqrt();
        self.scale(inv_length)
    }
}

impl fmt::Show for Vec3 {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "[{}, {}, {}]", self.v[0], self.v[1], self.v[2])
    }
}

impl Mul<Vec3, Vec3> for Vec3 {
    fn mul(&self, rhs: &Vec3) -> Vec3 {
        Vec3 { v: [
            self.v[0] * rhs.v[0],
            self.v[1] * rhs.v[1],
            self.v[2] * rhs.v[2]]
        }
    }
}

impl Add<Vec3, Vec3> for Vec3 {
    fn add(&self, rhs: &Vec3) -> Vec3 {
        Vec3 { v: [
            self.v[0] + rhs.v[0],
            self.v[1] + rhs.v[1],
            self.v[2] + rhs.v[2]]
        }
    }
}

impl Sub<Vec3, Vec3> for Vec3 {
    fn sub(&self, rhs: &Vec3) -> Vec3 {
        Vec3 { v: [
            self.v[0] - rhs.v[0],
            self.v[1] - rhs.v[1],
            self.v[2] - rhs.v[2]]
        }
    }
}

impl Index<uint, f32> for Vec3 {
    fn index<'a>(&'a self, i: &uint) -> &'a f32 {
        &self.v[*i]
    }
}

#[allow(dead_code)]
pub struct Vec4 {
    v: [f32, ..4]
}

#[allow(dead_code)]
impl Vec4 {
    pub fn new(x: f32, y: f32, z: f32, w: f32) -> Vec4 {
        Vec4 { v: [x, y, z, w] }
    }
    pub fn zero() -> Vec4 {
        Vec4 { v: [0.0, 0.0, 0.0, 0.0] }
    }
    pub fn identity() -> Vec4 {
        Vec4 { v: [1.0, 1.0, 1.0, 1.0] }
    }
    pub fn scale(&self, s: f32) -> Vec4 {
        Vec4 { v: [
            self.v[0] * s,
            self.v[1] * s,
            self.v[2] * s,
            self.v[3] * s]
        }
    }
    pub fn dot(&self, rhs: &Vec4) -> f32 {
        (self.v[0] * rhs.v[0]) + (self.v[1] * rhs.v[1]) +
            (self.v[2] * rhs.v[2]) + (self.v[3] * rhs.v[3])
    }
    pub fn length(&self) -> f32 {
        self.dot(self).sqrt()

    }
    pub fn normalize(&self) -> Vec4 {
        let inv_length = self.dot(self).rsqrt();
        self.scale(inv_length)
    }
}

/*
impl Mul<f32, Vec4> for Vec4 {
    fn mul(&self, rhs: &f32) -> Vec4 {
        Vec4::zero()
    }
}
*/

impl Mul<Vec4, Vec4> for Vec4 {
    fn mul(&self, rhs: &Vec4) -> Vec4 {
        Vec4 { v: [
            self.v[0] * rhs.v[0],
            self.v[1] * rhs.v[1],
            self.v[2] * rhs.v[2],
            self.v[3] * rhs.v[3]]
        }
    }
}

impl Add<Vec4, Vec4> for Vec4 {
    fn add(&self, rhs: &Vec4) -> Vec4 {
        Vec4 { v: [
            self.v[0] + rhs.v[0],
            self.v[1] + rhs.v[1],
            self.v[2] + rhs.v[2],
            self.v[3] + rhs.v[3]]
        }
    }
}

impl Sub<Vec4, Vec4> for Vec4 {
    fn sub(&self, rhs: &Vec4) -> Vec4 {
        Vec4 { v: [
            self.v[0] - rhs.v[0],
            self.v[1] - rhs.v[1],
            self.v[2] - rhs.v[2],
            self.v[3] - rhs.v[3]]
        }
    }
}

impl Index<uint, f32> for Vec4 {
    fn index<'a>(&'a self, i: &uint) -> &'a f32 {
        &self.v[*i]
    }
}

impl fmt::Show for Vec4 {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "[{}, {}, {}, {}]", self.v[0], self.v[1], self.v[2], self.v[3])
    }
}

#[allow(dead_code)]
pub struct Mat4 {
    col: [Vec4, ..4]
}

#[allow(dead_code)]
impl Mat4 {
    pub fn as_ptr(&self) -> *const f32 {
        &self.col[0][0] as *const f32
    }

    pub fn new(col0: &Vec4, col1: &Vec4, col2: &Vec4, col3: &Vec4) -> Mat4 {
        Mat4 { col: [ *col0, *col1, *col2, *col3 ] }
    }

    pub fn zero() -> Mat4 {
        Mat4 { col: [ Vec4::zero(), Vec4::zero(), Vec4::zero(), Vec4::zero() ] }
    }

    pub fn identity() -> Mat4 {
        Mat4 { col: [
            Vec4::new(1.0, 0.0, 0.0, 0.0),
            Vec4::new(0.0, 1.0, 0.0, 0.0),
            Vec4::new(0.0, 0.0, 1.0, 0.0),
            Vec4::new(0.0, 0.0, 0.0, 1.0)]
        }
    }

    pub fn perspective(fovy: f32, aspect: f32, near: f32, far: f32) -> Mat4 {
        let q = 1.0 / (0.5 * fovy).to_radians().tan();
        let a = q / aspect;
        let b = (near + far) / (near - far);
        let c = (2.0 * near * far) / (near - far);

        Mat4 { col: [
            Vec4::new(a, 0.0, 0.0, 0.0),
            Vec4::new(0.0, q, 0.0, 0.0),
            Vec4::new(0.0, 0.0, b, -1.0),
            Vec4::new(0.0, 0.0, c, 0.0)]
        }
    }

    pub fn translate(x: f32, y: f32, z: f32) -> Mat4 {
        Mat4 { col: [
            Vec4::new(1.0, 0.0, 0.0, 0.0),
            Vec4::new(0.0, 1.0, 0.0, 0.0),
            Vec4::new(0.0, 0.0, 1.0, 0.0),
            Vec4::new(x, y, z, 1.0)]
        }
    }

    pub fn lookat(eye: &Vec3, center: &Vec3, up: &Vec3) -> Mat4 {
        let f = (*center - *eye).normalize();
        let up_n = up.normalize();
        let s = f.cross(&up_n);
        let u = s.cross(&f);
        Mat4 { col: [
            Vec4::new(s[0], u[0], -f[0], 0.0),
            Vec4::new(s[1], u[1], -f[1], 0.0),
            Vec4::new(s[2], u[2], -f[2], 0.0),
            Vec4::new(0.0, 0.0, 0.0, 1.0)]
        } * Mat4::translate(-eye[0], -eye[1], -eye[2])
    }

    pub fn scale(x: f32, y: f32, z: f32) -> Mat4 {
        Mat4 { col: [
            Vec4::new(  x, 0.0, 0.0, 0.0),
            Vec4::new(0.0,   y, 0.0, 0.0),
            Vec4::new(0.0, 0.0,   z, 0.0),
            Vec4::new(0.0, 0.0, 0.0, 1.0)]
        }
    }

    pub fn rotate(angle: f32, x: f32, y: f32, z: f32) -> Mat4 {
        let x2 = x * x;
        let y2 = y * y;
        let z2 = z * z;
        let rads = angle.to_radians();
        let (s, c) = rads.sin_cos();
        let omc = 1.0 - c;
        Mat4 { col: [
            Vec4::new(x2 * omc + c, y * x * omc + z * s, x * z * omc - y * s, 0.0),
            Vec4::new(x * y * omc - z * s, y2 * omc + c, y * z * omc + x * s, 0.0),
            Vec4::new(x * z * omc + y * s, y * z * omc - x * s, z2 * omc + c, 0.0),
            Vec4::new(0.0, 0.0, 0.0, 1.0)]
        }
    }
}

impl Mul<Mat4, Mat4> for Mat4 {
    fn mul(&self, rhs: &Mat4) -> Mat4 {
        let a0 = self.col[0];
        let a1 = self.col[1];
        let a2 = self.col[2];
        let a3 = self.col[3];

        let b0 = rhs.col[0];
        let b1 = rhs.col[1];
        let b2 = rhs.col[2];
        let b3 = rhs.col[3];

        Mat4 { col: [
            a0.scale(b0[0]) + a1.scale(b0[1]) + a2.scale(b0[2]) + a3.scale(b0[3]),
            a0.scale(b1[0]) + a1.scale(b1[1]) + a2.scale(b1[2]) + a3.scale(b1[3]),
            a0.scale(b2[0]) + a1.scale(b2[1]) + a2.scale(b2[2]) + a3.scale(b2[3]),
            a0.scale(b3[0]) + a1.scale(b3[1]) + a2.scale(b3[2]) + a3.scale(b3[3])]
        }
    }
}

impl Index<uint, Vec4> for Mat4 {
    fn index<'a>(&'a self, i: &uint) -> &'a Vec4 {
        &self.col[*i]
    }
}

impl fmt::Show for Mat4 {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "[{}, {}, {}, {}]",
           self.col[0], self.col[1], self.col[2], self.col[3])
    }
}

