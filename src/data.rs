use super::GL;
use crate::shader::Program;
use glow::Context;
use std::os::raw::{c_int, c_uint};

#[derive(Copy, Clone, Debug)]
#[repr(C, packed)]
pub struct f32_ {
    pub d0: f32,
}

impl f32_ {
    pub fn new(d0: f32) -> f32_ {
        f32_ { d0 }
    }

    pub unsafe fn vertex_attrib_pointer(
        gl: &GL,
        stride: c_int,
        location: u32,
        offset: c_int,
    ) {
        gl.enable_vertex_attrib_array(location as c_uint);
        gl.vertex_attrib_pointer_f32(
            location as c_uint,
            1, // the number of components per generic vertex attribute
            glow::FLOAT, // data type
            false, // normalized (int-to-float conversion)
            stride as c_int,
            offset,
        );
    }
}

impl From<f32> for f32_ {
    fn from(other: f32) -> Self {
        f32_::new(other)
    }
}

// ----------------------

#[derive(Copy, Clone, Debug)]
#[repr(C, packed)]
pub struct i32_ {
    pub d0: i32,
}

impl i32_ {
    pub fn new(d0: i32) -> i32_ {
        i32_ { d0 }
    }

    pub unsafe fn vertex_attrib_pointer(
        gl: &GL,
        stride: c_int,
        location: u32,
        offset: c_int,
    ) {
        gl.enable_vertex_attrib_array(location as c_uint);
        gl.vertex_attrib_pointer_i32(
            location,
            1,         // the number of components per generic vertex attribute
            glow::INT, // data type
            stride as c_int,
            offset,
        );
    }
}

impl From<i32> for i32_ {
    fn from(other: i32) -> Self {
        i32_::new(other)
    }
}

#[allow(non_camel_case_types)]
#[derive(Copy, Clone, Debug)]
#[repr(C, packed)]
pub struct f32_f32_f32(pub f32, pub f32, pub f32);

impl f32_f32_f32 {
    pub fn new(d0: f32, d1: f32, d2: f32) -> f32_f32_f32 {
        f32_f32_f32(d0, d1, d2)
    }

    pub unsafe fn vertex_attrib_pointer(
        gl: &GL,
        stride: c_int,
        location: u32,
        offset: c_int,
    ) {
        gl.enable_vertex_attrib_array(location);
        gl.vertex_attrib_pointer_f32(
            location,
            3,
            glow::FLOAT,
            false,
            stride,
            offset,
        );
    }
}

impl From<(f32, f32, f32)> for f32_f32_f32 {
    fn from(other: (f32, f32, f32)) -> Self {
        f32_f32_f32::new(other.0, other.1, other.2)
    }
}

#[allow(non_camel_case_types)]
#[derive(Copy, Clone, Debug)]
#[repr(C, packed)]
pub struct f32_f32_f32_f32(pub f32, pub f32, pub f32, pub f32);

impl f32_f32_f32_f32 {
    pub fn new(d0: f32, d1: f32, d2: f32, d3: f32) -> f32_f32_f32_f32 {
        f32_f32_f32_f32(d0, d1, d2, d3)
    }

    pub unsafe fn vertex_attrib_pointer(
        gl: &GL,
        stride: c_int,
        location: u32,
        offset: c_int,
    ) {
        gl.enable_vertex_attrib_array(location);
        gl.vertex_attrib_pointer_f32(
            location,
            4,
            glow::FLOAT,
            false,
            stride,
            offset,
        );
    }
}

impl From<(f32, f32, f32, f32)> for f32_f32_f32_f32 {
    fn from(other: (f32, f32, f32, f32)) -> Self {
        f32_f32_f32_f32::new(other.0, other.1, other.2, other.3)
    }
}

#[allow(non_camel_case_types)]
#[derive(Copy, Clone, Debug)]
#[repr(C, packed)]
pub struct f32_f32 {
    pub d0: f32,
    pub d1: f32,
}

impl f32_f32 {
    pub fn new(d0: f32, d1: f32) -> f32_f32 {
        f32_f32 { d0, d1 }
    }

    pub unsafe fn vertex_attrib_pointer(
        gl: &GL,
        stride: c_int,
        location: u32,
        offset: c_int,
    ) {
        gl.enable_vertex_attrib_array(location);
        gl.vertex_attrib_pointer_f32(
            location,
            2,
            glow::FLOAT,
            false,
            stride,
            offset,
        );
    }
}

impl From<(f32, f32)> for f32_f32 {
    fn from(other: (f32, f32)) -> Self {
        f32_f32::new(other.0, other.1)
    }
}

pub trait Vertex {
    fn vertex_attrib_pointers(gl: &GL, program: &Program);
}
