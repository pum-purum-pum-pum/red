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

pub trait ContextTypes {
    
}

#[derive(Clone)]
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

pub enum DrawType {
    Standart,
    Instancing(usize)
}

pub struct Frame {
    gl: GL
}

impl Frame {
    pub fn new(gl: &GL) -> Frame {
        Frame {
            gl: gl.clone()
        }
    }

    pub fn draw(
        &self,
        vao: &buffer::VertexArray,
        index_buffer: Option<&buffer::IndexBuffer>,
        program: &Program,
        draw_type: &DrawType
    ) {
        program.set_used();
        vao.bind();
        match index_buffer {
            Some(index_buffer) => {
                index_buffer.bind();
                unsafe {
                    // dbg!(index_buffer.size);
                    match draw_type {
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
                                *instance_count as i32
                            )
                        }
                    }
                }
            }
            None => {
                unimplemented!();
            }
        }
    }
    pub fn set_clear_color(&self, red: f32, green: f32, blue: f32, alpha: f32) {
        unsafe{self.gl.clear_color(red, green, blue, alpha)};
    }

    pub fn clear_color(&self) {
        unsafe {self.gl.clear(glow::COLOR_BUFFER_BIT)};
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
}

pub use shader::*;
pub use data::*;