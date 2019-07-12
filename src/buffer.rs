use super::GL;
use super::shader;
use glow;
use glow::Context;
use std::os::raw::{c_uint};

pub struct IndexBuffer {
    veb: ElementArrayBuffer,
    pub size: usize
}

impl IndexBuffer {
    pub fn new(gl: &GL, index: &[u16]) -> Result<IndexBuffer, String> {
        let veb = ElementArrayBuffer::new(&gl)?;
        veb.bind();
        veb.element_draw_data(index); // is it safe without thoose binds
        veb.unbind();
        Ok(IndexBuffer{veb: veb, size: index.len()})
    }

    pub fn bind(&self) {
        self.veb.bind();
    }

    // pub fn unbind(&self) {
    //     self.veb.unbind()
    // }
}

pub trait VertexBufferBehavior {
    fn bind(&self);

    fn unbind(&self);

    fn vertex_attrib_pointers(&self, gl: &GL, program: &shader::Program);
}

pub trait BufferType {
    const BUFFER_TYPE: c_uint;
}

pub struct BufferTypeArray;
impl BufferType for BufferTypeArray {
    const BUFFER_TYPE: c_uint = glow::ARRAY_BUFFER;
}

pub struct BufferTypeElementArray;
impl BufferType for BufferTypeElementArray {
    const BUFFER_TYPE: c_uint = glow::ELEMENT_ARRAY_BUFFER;
}

pub struct Buffer<B>
where
    B: BufferType,
{
    gl: GL,
    vbo: c_uint,
    _marker: ::std::marker::PhantomData<B>,
}

impl<B> Buffer<B>
where
    B: BufferType,
{
    pub fn new(gl: &GL) -> Result<Buffer<B>, String> {
        let vbo = unsafe{gl.create_buffer()?};
        Ok(Buffer {
            gl: gl.clone(),
            vbo,
            _marker: ::std::marker::PhantomData,
        })
    }

    pub fn bind(&self) {
        unsafe {
            self.gl.bind_buffer(B::BUFFER_TYPE, Some(self.vbo));
        }
    }

    pub fn unbind(&self) {
        unsafe {
            self.gl.bind_buffer(B::BUFFER_TYPE, None);
        }
    }

    pub fn static_draw_data<T>(&self, data: &[T]) {
        unsafe {
            self.gl.buffer_data_u8_slice(
                glow::ARRAY_BUFFER, 
                std::slice::from_raw_parts(
                    data.as_ptr() as *const u8, 
                    data.len() * std::mem::size_of::<T>()
                ),
                glow::STATIC_DRAW
            );
        }
    }

    pub fn element_draw_data(&self, data: &[u16]) {
        unsafe {
            self.gl.buffer_data_u8_slice(
                glow::ELEMENT_ARRAY_BUFFER,
                std::slice::from_raw_parts(
                    data.as_ptr() as *const u8, 
                    data.len() * std::mem::size_of::<u16>()
                ),
                glow::STATIC_DRAW
            );
        }
    }
}

impl<B> Drop for Buffer<B>
where
    B: BufferType,
{
    fn drop(&mut self) {
        unsafe {
            self.gl.delete_buffer(self.vbo)
        }
    }
}

pub type ArrayBuffer = Buffer<BufferTypeArray>;
pub type ElementArrayBuffer = Buffer<BufferTypeElementArray>;

pub struct VertexArray {
    gl: GL,
    vao: c_uint,
}

impl VertexArray {
    pub fn new(gl: &GL) -> Result<VertexArray, String> {
        let vao = unsafe {gl.create_vertex_array()?};
        Ok(VertexArray {
            gl: gl.clone(),
            vao,
        })
    }

    pub fn bind(&self) {
        unsafe {
            self.gl.bind_vertex_array(Some(self.vao));
        }
    }

    pub fn unbind(&self) {
        unsafe {
            self.gl.bind_vertex_array(None);
        }
    }
}

impl Drop for VertexArray {
    fn drop(&mut self) {
        unsafe {
            self.gl.delete_vertex_array(self.vao);
        }
    }
}