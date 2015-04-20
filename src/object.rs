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

use gl::types::*;
use std::fs;
use std::io;
use std::io::Read;
use std::mem;
use std::path::Path;
use std::ptr;
use std::str;
use reader::BufferReader;

macro_rules! fourcc(
    ($a:expr, $b:expr, $c:expr, $d:expr) => (
        (($a as u32) << 0 | ($b as u32) << 8 | ($c as u32) << 16 | ($d as u32) <<24)
    );
);

const INDEX_DATA_TYPE: u32 = fourcc!('I','N','D','X');
const VERTEX_DATA_TYPE: u32 = fourcc!('V','R','T','X');
const VERTEX_ATTRIBS_TYPE: u32 = fourcc!('A','T','R','B');
const SUB_OBJECT_LIST_TYPE: u32 = fourcc!('O','L','S','T');
const COMMENT_TYPE: u32 = fourcc!('C','M','N','T');

const VERTEX_ATTRIB_FLAG_NORMALIZED: u32 = 0x00000001;

struct MeshHeader {
    size: u32,
    num_chunks: u32,
    flags: u32
}

struct ChunkHeader {
    chunk_type: u32,
    size: u32
}

struct IndexData {
    index_type: u32,
    index_count: u32,
    index_data_offset: u32
}

struct VertexData {
    data_size: u32,
    data_offset: u32,
    total_vertices: u32
}

struct VertexAttribDecl {
    #[allow(dead_code)]
    name: [u8; 64],
    size: u32,
    ty: u32,
    stride: u32,
    flags: u32,
    data_offset: u32
}

#[derive(Debug)]
pub enum LoadError {
    MagicError(Option<String>),
    ChunkTypeError(u32),
    ChunkSizeError(usize, usize),
    VertexDataError,
    VertexAttribDataError,
    IoError(io::Error),
}

impl From<io::Error> for LoadError {
    fn from(e: io::Error) -> LoadError {
        LoadError::IoError(e)
    }
}

#[derive(Clone, Copy)]
struct SubObjectDecl {
    first: u32,
    count: u32
}

pub struct Object {
    vertex_buffer: GLuint,
    index_buffer: GLuint,
    vao: GLuint,
    num_indices: GLuint,
    index_type: GLuint,
    num_sub_objects: usize,
    sub_object: [SubObjectDecl; 256]
}

// TODO: #[derive(Clone)] is currently limited to arrays up to 32 length
impl Clone for Object {
    #[inline]
    fn clone(&self) -> Object { *self }
}

impl Copy for Object {
}

impl Object {
    pub fn new() -> Object {
        Object {
            vertex_buffer: 0,
            index_buffer: 0,
            vao: 0,
            num_indices: 0,
            index_type: 0,
            num_sub_objects: 0,
            sub_object: [SubObjectDecl { first: 0, count: 0 }; 256]
        }
    }

    pub fn load(&mut self, filename: &str) -> Result<(), LoadError> {
        let mut file = try!(fs::File::open(&Path::new(filename)));
        let mut bytes = Vec::new();
        try!(file.read_to_end(&mut bytes));

        let mut reader = BufferReader::new(bytes);
        let mut bytes_read = 0;

        // check header magic
        let magic = try!(reader.pop_slice::<u8>(4));
        match str::from_utf8(magic) {
            Ok(v) if v == "SB6M" => (),
            Ok(v) => return Err(LoadError::MagicError(Some(String::from(v)))),
            Err(_) => return Err(LoadError::MagicError(None))
        }

        debug!("magic: {}", str::from_utf8(magic).unwrap());

        let header = try!(reader.pop_value::<MeshHeader>());
        bytes_read += header.size as usize;

        debug!("size: {}, num_chunks: {}, flags: {}",
            header.size, header.num_chunks, header.flags);
        assert!(bytes_read == reader.bytes_read());

        let mut vertex_attrib_data_ref: Option<&[VertexAttribDecl]> = None;
        let mut vertex_data_chunk_ref: Option<&VertexData> = None;
        let mut index_data_chunk_ref: Option<&IndexData> = None;
        let mut sub_object_data_ref: Option<&[SubObjectDecl]> = None;

        for _ in 0..header.num_chunks {
            let chunk_header = try!(reader.pop_value::<ChunkHeader>());
            match chunk_header.chunk_type {
                INDEX_DATA_TYPE => {
                    debug!("INDX");
                    // read in index data struct
                    index_data_chunk_ref = Some(
                        try!(reader.pop_value::<IndexData>()));
                }
                VERTEX_DATA_TYPE => {
                    debug!("VRTX");
                    // read in vertex data struct
                    vertex_data_chunk_ref = Some(
                        try!(reader.pop_value::<VertexData>()));
                },
                VERTEX_ATTRIBS_TYPE => {
                    debug!("ATRB");
                    // read attribute count
                    let attrib_count = try!(reader.pop_value::<u32>());
                    // read in all the attributes
                    vertex_attrib_data_ref = Some(
                        try!(reader.pop_slice::<VertexAttribDecl>(
                                *attrib_count as usize)));
                },
                SUB_OBJECT_LIST_TYPE => {
                    debug!("OLST");
                    // read sub object count
                    let sub_object_count = try!(reader.pop_value::<u32>());
                    debug!("sub_object_count: {}", sub_object_count);
                    // read in sub object data
                    sub_object_data_ref = Some(
                        try!(reader.pop_slice::<SubObjectDecl>(
                                *sub_object_count as usize)));
                },
                COMMENT_TYPE => {
                    debug!("CMNT");
                    let comment_len = chunk_header.size as usize -
                        mem::size_of::<ChunkHeader>();
                    let comment_bytes_ref = try!(reader.pop_slice::<u8>(
                            comment_len));
                    match str::from_utf8(comment_bytes_ref) {
                        Ok(v) => debug!("{}", v),
                        _ => panic!("couldn't read comment")
                    };
                },
                _ => return Err(LoadError::ChunkTypeError(chunk_header.chunk_type))
            }
            bytes_read += chunk_header.size as usize;
            assert!(bytes_read == reader.bytes_read());
        }

        // check the expected number of bytes read
        if bytes_read != reader.bytes_read() {
            return Err(LoadError::ChunkSizeError(bytes_read, reader.bytes_read()))
        }

        // vertex data required
        let vertex_data_chunk = match vertex_data_chunk_ref {
            Some(v) => v,
            None => return Err(LoadError::VertexDataError)
        };

        // vertex attribute required
        let vertex_attrib_data = match vertex_attrib_data_ref {
            Some(v) => v,
            None => return Err(LoadError::VertexAttribDataError)
        };

        match sub_object_data_ref {
            Some(sub_object_data) => {
                debug!("sub_object_count: {}", sub_object_data.len());
                self.num_sub_objects = sub_object_data.len();
                for i in 0..self.num_sub_objects {
                    self.sub_object[i] = sub_object_data[i];
                }
            },
            None => {
                self.num_sub_objects = 1;
                self.sub_object[0].count = vertex_data_chunk.total_vertices;
            }
        }

        // bind vertex data
        let vertex_data_start = vertex_data_chunk.data_offset as usize;
        let vertex_data_end = vertex_data_start + vertex_data_chunk.data_size as usize;
        let vertex_data = try!(reader.peek_slice(vertex_data_start, vertex_data_end));
        unsafe {
            gl::GenBuffers(1, &mut self.vertex_buffer);
            gl::BindBuffer(gl::ARRAY_BUFFER, self.vertex_buffer);
            gl::BufferData(gl::ARRAY_BUFFER,
                           vertex_data_chunk.data_size as GLsizeiptr,
                           mem::transmute(vertex_data.as_ptr()),
                           gl::STATIC_DRAW);
            gl::GenVertexArrays(1, &mut self.vao);
            gl::BindVertexArray(self.vao);
        }

        // bind vertex attributes
        for i in 0..vertex_attrib_data.len() {
            let attrib_decl = &vertex_attrib_data[i];
            let attrib_flags =
                if attrib_decl.flags & VERTEX_ATTRIB_FLAG_NORMALIZED != 0 {
                    gl::TRUE
                } else {
                    gl::FALSE
                };
            let attrib_data_offset = attrib_decl.data_offset as usize;
            unsafe {
                gl::VertexAttribPointer(i as u32,
                                        attrib_decl.size as i32,
                                        attrib_decl.ty,
                                        attrib_flags,
                                        attrib_decl.stride as i32,
                                        mem::transmute(attrib_data_offset));
                gl::EnableVertexAttribArray(i as u32);
            }
        }

        // bind index data
        match index_data_chunk_ref {
            Some(index_data_chunk) => {
                let indice_size = 
                    if index_data_chunk.index_type == gl::UNSIGNED_SHORT as u32 {
                        mem::size_of::<GLushort>()
                    } else {
                        mem::size_of::<GLubyte>()
                    };
                let index_data_size =
                    index_data_chunk.index_count as usize * indice_size;
                let index_data_start =
                    index_data_chunk.index_data_offset as usize;
                let index_data_end = index_data_start + index_data_size;
                let index_data = try!(reader.peek_slice(index_data_start,
                                                        index_data_end));
                unsafe {
                    gl::GenBuffers(1, &mut self.index_buffer);
                    gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, self.index_buffer);
                    gl::BufferData(gl::ELEMENT_ARRAY_BUFFER,
                                   index_data_size as GLsizeiptr,
                                   mem::transmute(index_data.as_ptr()),
                                   gl::STATIC_DRAW);
                }
                self.num_indices = index_data_chunk.index_count;
                self.index_type = index_data_chunk.index_type;
            },
            None => {
                self.num_indices = vertex_data_chunk.total_vertices;
            }
        }

        unsafe {
            gl::BindVertexArray(0);
            gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, 0);
        }

        Ok(())
    }

    pub fn free(&mut self) {
        unsafe {
            gl::DeleteVertexArrays(1, &self.vao);
            gl::DeleteBuffers(1, &self.vertex_buffer);
            gl::DeleteBuffers(1, &self.index_buffer);
        }

        self.vao = 0;
        self.vertex_buffer = 0;
        self.index_buffer = 0;
        self.num_indices = 0;
    }

    pub fn render(&self) {
        self.render_instances(1, 0);
    }

    pub fn render_instances(&self, instance_count: u32, base_instance: u32) {
        self.render_sub_object(0, instance_count, base_instance);
    }

    pub fn render_sub_object(&self, object_index: u32, instance_count: u32,
                             base_instance: u32) {

        unsafe {
            gl::BindVertexArray(self.vao);

            if self.index_buffer != 0 {
                gl::DrawElementsInstancedBaseInstance(
                    gl::TRIANGLES,
                    self.num_indices as i32,
                    self.index_type,
                    ptr::null(),
                    instance_count as i32,
                    base_instance);
            } else {
                gl::DrawArraysInstancedBaseInstance(
                    gl::TRIANGLES,
                    self.sub_object[object_index as usize].first as i32,
                    self.sub_object[object_index as usize].count as i32,
                    instance_count as i32,
                    base_instance);
            }
        }
    }
}
