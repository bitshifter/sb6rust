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

pub use glam::{deg, vec3, vec4, Mat4, Vec3, Vec4};
use std::f32;

#[allow(dead_code)]
pub fn perspective(fovy: f32, aspect: f32, near: f32, far: f32) -> Mat4 {
    // glu perspective
    let inv_length = 1.0 / (near - far);
    let f = 1.0 / (0.5 * fovy.to_radians()).tan();
    let a = f / aspect;
    let q = f;
    let b = (near + far) * inv_length;
    let c = (2.0 * near * far) * inv_length;
    Mat4::new(
        Vec4::new(a, 0.0, 0.0, 0.0),
        Vec4::new(0.0, q, 0.0, 0.0),
        Vec4::new(0.0, 0.0, b, -1.0),
        Vec4::new(0.0, 0.0, c, 0.0),
    )
}

#[allow(dead_code)]
pub fn translate(x: f32, y: f32, z: f32) -> Mat4 {
    Mat4::from_translation(vec3(x, y, z))
}

#[allow(dead_code)]
pub fn look_at(eye: Vec3, center: Vec3, up: Vec3) -> Mat4 {
    Mat4::look_at_lh(eye, center, up.normalize())
}

#[allow(dead_code)]
pub fn rotate(angle: f32, x: f32, y: f32, z: f32) -> Mat4 {
    Mat4::from_axis_angle(vec3(x, y, z), deg(angle))
}

#[allow(dead_code)]
pub fn scale(x: f32, y: f32, z: f32) -> Mat4 {
    Mat4::from_scale(vec3(x, y, z))
}

#[allow(dead_code)]
pub fn identity() -> Mat4 {
    Mat4::identity()
}
