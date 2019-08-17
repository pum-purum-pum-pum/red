use std::rc::Rc;
use std::ops::Deref;
#[cfg(not(target_arch = "wasm32"))]
use glow::native::Context as GL_Context;
use glow::Context;
extern crate vertex_derive;
pub use vertex_derive::VertexAttribPointers;

pub mod shader;
pub mod data;
pub mod buffer;
pub use glow;

use std::os::raw::{c_uint, c_int};

pub trait ContextTypes {
    
}

#[derive(Clone, Debug)]
pub struct GL {
    inner: Rc<GL_Context>
}

impl GL {
    pub fn new(context: GL_Context) -> GL {
        GL {
            inner: Rc::new(context)
        }
    }
}

impl Deref for GL {
    type Target = GL_Context;
    fn deref(&self) -> &GL_Context {
        &self.inner
    }
}

#[derive(Debug, Clone,Copy)]
pub enum Operation {
    Replace
}

#[derive(Debug, Clone)]
pub enum StencilTest {
    AlwaysPass,
    Equal,
    NotEqual
}

impl Default for StencilTest {
    fn default() -> Self {
        StencilTest::AlwaysPass
    }
}

#[derive(Debug, Default, Clone)]
pub struct Stencil {
    pub mask: c_uint,
    pub ref_value: c_int,
    pub test: StencilTest,
    pub pass_operation: Option<Operation>,
}

#[derive(Debug)]
pub enum DrawType {
    Standart,
    Instancing(usize),
}

#[derive(Debug, Default)]
pub struct Blend;

impl Default for DrawType {
    fn default() -> Self {
        DrawType::Standart
    }
}

#[derive(Debug)]
pub struct DrawParams {
    pub stencil: Option<Stencil>,
    pub draw_type: DrawType,
    pub color_mask: (bool, bool, bool, bool),
    pub blend: Option<Blend>
}

impl Default for DrawParams {
    fn default() -> Self {
        Self {
            stencil: None,
            draw_type: DrawType::default(),
            color_mask: (true, true, true, true),
            blend: Some(Blend)
        }
    }
}

pub struct Frame {
    pub gl: GL
}

impl Frame {
    pub fn new(gl: &GL) -> Frame {
        unsafe {
            gl.enable(glow::STENCIL_TEST); // TODO: Should it be here?
        }
        Frame {
            gl: gl.clone()
        }
    }

    pub fn draw(
        &self,
        vao: &buffer::VertexArray,
        index_buffer: Option<&buffer::IndexBuffer>,
        program: &Program,
        draw_params: &DrawParams
    ) {
        vao.bind();
        if let Some(blend) = &draw_params.blend {
            unsafe {
            // TODO make as param
                self.gl.enable(glow::BLEND);
                self.gl.blend_func(glow::SRC_ALPHA, glow::ONE_MINUS_SRC_ALPHA);
                // self.gl.enable(glow::STENCIL_TEST); // TODO: Should it be here?
            }
        } else {
            unsafe {
                // strange
                // self.gl.disable(glow::BLEND);
            }
        }
        unsafe {
            self.gl.color_mask(
                draw_params.color_mask.0, 
                draw_params.color_mask.1, 
                draw_params.color_mask.2, 
                draw_params.color_mask.3
            );
        }
        if let Some(stencil) = draw_params.stencil.clone() {
            unsafe{
                self.gl.stencil_mask(0xFF);
            }
            match stencil.test {
                StencilTest::AlwaysPass => {
                    unsafe {
                        self.gl.stencil_func(
                            glow::ALWAYS, 
                            stencil.ref_value, 
                            stencil.mask
                        );
                    }
                },
                StencilTest::Equal => {
                    unsafe {
                        self.gl.stencil_func(
                            glow::EQUAL, 
                            stencil.ref_value, 
                            stencil.mask
                        )
                    }
                },
                StencilTest::NotEqual => {
                    unsafe {
                        self.gl.stencil_func(
                            glow::NOTEQUAL,
                            stencil.ref_value,
                            stencil.mask
                        )
                    }
                }
            }
            let mut pass_operation = glow::KEEP;
            let fail_operation = glow::KEEP;
            let depth_fail_operation = glow::KEEP;
            if let Some(operation) = stencil.pass_operation {
                match operation {
                    Operation::Replace => {
                        pass_operation = glow::REPLACE;
                    }
                }
            }
            unsafe {
                self.gl.stencil_op(
                    fail_operation, 
                    depth_fail_operation, 
                    pass_operation
                )
            }
        } else {
            unsafe{
                self.gl.stencil_mask(0x00);
                self.gl.stencil_func(glow::ALWAYS, 0, 0xFF);
                // TODO: just deltete?
                self.gl.stencil_op(
                    glow::KEEP, 
                    glow::KEEP, 
                    glow::KEEP
                )
            }
        };
        program.set_used();
        unsafe{
            match index_buffer {
                Some(index_buffer) => {
                    index_buffer.bind();
                    match draw_params.draw_type {
                        DrawType::Standart => {
                            self.gl.draw_elements(
                                glow::TRIANGLES, 
                                index_buffer.size as i32, 
                                glow::UNSIGNED_SHORT, 
                                0
                            );
                        }
                        DrawType::Instancing(instance_count) => {
                            self.gl.draw_elements_instanced(
                                glow::TRIANGLES, 
                                index_buffer.size as i32, 
                                glow::UNSIGNED_SHORT, 
                                0,
                                instance_count as i32
                            )
                        }
                    }
            }
            None => {
                unimplemented!();
            }
        }
        self.gl.stencil_mask(0xFF); // oh it's so painfull
    }   
        vao.unbind();
    }
    pub fn set_clear_stencil(&self, stencil: i32) {
        unsafe{self.gl.clear_stencil(stencil)};
    }

    pub fn set_clear_color(&self, red: f32, green: f32, blue: f32, alpha: f32) {
        unsafe{self.gl.clear_color(red, green, blue, alpha)};
    }

    pub fn clear_color(&self) {
        unsafe {self.gl.clear(glow::COLOR_BUFFER_BIT)};
    }

    pub fn clear_stencil(&self) {
        unsafe {self.gl.clear(glow::STENCIL_BUFFER_BIT)};
    }

    pub fn clear_color_and_stencil(&self) {
        unsafe {self.gl.clear(glow::COLOR_BUFFER_BIT | glow::STENCIL_BUFFER_BIT)};
    }
}

pub struct Viewport {
    pub x: i32,
    pub y: i32,
    pub w: i32,
    pub h: i32,
}

impl Viewport {
    pub fn for_window(w: i32, h: i32) -> Viewport {
        Viewport {
            x: 0,
            y: 0,
            w,
            h
        }
    }

    pub fn update_size(&mut self, w: i32, h: i32) {
        self.w = w;
        self.h = h;
    }

    pub fn set_used(&self, gl: &GL) {
        unsafe {
            gl.viewport(self.x, self.y, self.w, self.h);
        }
    }

    pub fn dimensions(&self) -> (i32, i32) {
        (self.w, self.h)
    }
}

pub use shader::*;
pub use data::*;