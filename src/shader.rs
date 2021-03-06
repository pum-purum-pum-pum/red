use std::os::raw::{c_int, c_uint};

use super::buffer::{VertexArray, VertexBufferBehavior};
use super::GL;
use glow;
use glow::native::Context as GL_Context;
use glow::Context;

#[derive(Clone, Debug)]
pub struct Texture {
    pub texture: <GL_Context as Context>::Texture,
    w: u32,
    h: u32,
}

impl PartialEq for Texture {
    fn eq(&self, other: &Texture) -> bool {
        self.texture == other.texture
    }
}

impl Texture {
    pub fn from_rgba8(
        gl_ctx: &GL,
        width: u32,
        height: u32,
        bytes: &[u8],
    ) -> Texture {
        unsafe {
            let texture = gl_ctx.create_texture().unwrap();
            gl_ctx.active_texture(glow::TEXTURE0);
            gl_ctx.bind_texture(glow::TEXTURE_2D, Some(texture));
            gl_ctx.tex_image_2d(
                glow::TEXTURE_2D,
                0,
                glow::RGBA as i32,
                width as i32,
                height as i32,
                0,
                glow::RGBA,
                glow::UNSIGNED_BYTE,
                Some(bytes),
            );
            gl_ctx.tex_parameter_i32(
                glow::TEXTURE_2D,
                glow::TEXTURE_WRAP_S,
                glow::CLAMP_TO_EDGE as i32,
            );
            gl_ctx.tex_parameter_i32(
                glow::TEXTURE_2D,
                glow::TEXTURE_WRAP_T,
                glow::CLAMP_TO_EDGE as i32,
            );
            gl_ctx.tex_parameter_i32(
                glow::TEXTURE_2D,
                glow::TEXTURE_MAG_FILTER,
                glow::NEAREST as i32,
            );
            gl_ctx.tex_parameter_i32(
                glow::TEXTURE_2D,
                glow::TEXTURE_MIN_FILTER,
                glow::NEAREST as i32,
            );
            gl_ctx.bind_texture(glow::TEXTURE_2D, None);
            Texture {
                texture,
                w: width,
                h: height,
            }
        }
    }

    pub fn dimensions(&self) -> (u32, u32) {
        (self.w, self.h)
    }

    pub fn new(gl_ctx: &GL, (width, height): (u32, u32)) -> Self {
        let mut name = 0;
        unsafe {
            // Create a texture for the glyphs
            // The texture holds 1 byte per pixel as alpha data
            gl_ctx.pixel_store_i32(glow::UNPACK_ALIGNMENT, 1);
            let name = gl_ctx.create_texture().unwrap();
            // gl_ctx.GenTextures(1, &mut name);
            gl_ctx.bind_texture(glow::TEXTURE_2D, Some(name));
            // gl_ctx.tex_parameter_i32(glow::TEXTURE_2D, glow::TEXTURE_WRAP_S, glow::CLAMP_TO_EDGE as _);
            // gl_ctx.tex_parameter_i32(glow::TEXTURE_2D, glow::TEXTURE_WRAP_T, glow::CLAMP_TO_EDGE as _);
            gl_ctx.tex_parameter_i32(
                glow::TEXTURE_2D,
                glow::TEXTURE_MIN_FILTER,
                glow::LINEAR as _,
            );
            gl_ctx.tex_parameter_i32(
                glow::TEXTURE_2D,
                glow::TEXTURE_MAG_FILTER,
                glow::NEAREST as _,
            );
            gl_ctx.tex_image_2d(
                glow::TEXTURE_2D,
                0,
                glow::RGB as _,
                width as _,
                height as _,
                0,
                glow::RGBA,
                glow::UNSIGNED_BYTE,
                None,
            );
            gl_ctx.bind_texture(glow::TEXTURE_2D, None);
            // gl_assert_ok!();

            Self {
                texture: name,
                w: width,
                h: height,
            }
        }
    }
}

// TODO
// impl Drop for Texture {
//     fn drop(&mut self) {
//         unsafe {

//         }
//     }
// }

pub struct Program {
    gl: GL,
    id: c_uint,
}

// Fedor(not-fl'3)'s uniforms code
pub trait UniformValue: Clone + PartialEq {
    fn set(self, gl: &GL, location: <GL_Context as Context>::UniformLocation);
}

impl UniformValue for (f32, f32, f32, f32) {
    fn set(self, gl: &GL, location: <GL_Context as Context>::UniformLocation) {
        unsafe {
            gl.uniform_4_f32(Some(location), self.0, self.1, self.2, self.3);
        }
    }
}

impl UniformValue for (f32, f32, f32) {
    fn set(self, gl: &GL, location: <GL_Context as Context>::UniformLocation) {
        unsafe {
            gl.uniform_3_f32(Some(location), self.0, self.1, self.2);
        }
    }
}

impl UniformValue for (f32, f32) {
    fn set(self, gl: &GL, location: <GL_Context as Context>::UniformLocation) {
        unsafe {
            gl.uniform_2_f32(Some(location), self.0, self.1);
        }
    }
}

impl UniformValue for f32 {
    fn set(self, gl: &GL, location: <GL_Context as Context>::UniformLocation) {
        unsafe {
            gl.uniform_1_f32(Some(location), self);
        }
    }
}

impl UniformValue for [[f32; 4]; 4] {
    fn set(self, gl: &GL, location: <GL_Context as Context>::UniformLocation) {
        unsafe {
            // transmute here according to
            // https://users.rust-lang.org/t/converting-f32-4-4-to-f32-16/22391
            // is safe
            gl.uniform_matrix_4_f32_slice(
                Some(location),
                false,
                &std::mem::transmute(self),
            )
        }
    }
}

impl UniformValue for [f32; 16] {
    fn set(self, gl: &GL, location: <GL_Context as Context>::UniformLocation) {
        unsafe { gl.uniform_matrix_4_f32_slice(Some(location), false, &self) }
    }
}

impl UniformValue for Texture {
    fn set(self, gl: &GL, location: <GL_Context as Context>::UniformLocation) {
        unsafe {
            gl.active_texture(glow::TEXTURE0);
            gl.bind_texture(glow::TEXTURE_2D, Some(self.texture));
            gl.uniform_1_i32(Some(location), 0);
        }
    }
}

impl Program {
    pub fn set_layout(
        &self,
        gl: &GL,
        vao: &VertexArray,
        vbos: &[&dyn VertexBufferBehavior],
    ) {
        vao.bind();
        for vbo in vbos.iter() {
            vbo.bind();
            vbo.vertex_attrib_pointers(&gl, &self);
            vbo.unbind();
        }
        vao.unbind();
    }

    pub fn from_shaders(
        gl: &GL,
        shaders: &[Shader],
    ) -> Result<Program, String> {
        let program_id = unsafe { gl.create_program()? };

        for shader in shaders {
            unsafe {
                gl.attach_shader(program_id, shader.id());
            }
        }

        unsafe {
            gl.link_program(program_id);
            if !gl.get_program_link_status(program_id) {
                return Err(gl.get_program_info_log(program_id));
            }
        }
        for shader in shaders {
            unsafe {
                // gl.DetachShader(program_id, shader.id());
                gl.detach_shader(program_id, shader.id());
            }
        }

        Ok(Program {
            gl: gl.clone(),
            id: program_id,
        })
    }

    pub fn id(&self) -> c_uint {
        self.id
    }

    pub fn set_used(&self) {
        unsafe {
            self.gl.use_program(Some(self.id));
        }
    }

    pub fn set_uniform<T: UniformValue>(&self, name: &str, uniform: T) {
        self.set_used();
        let location = unsafe {
            self.gl.get_uniform_location(self.id(), name)
            .expect(&format!("name \"{}\" does not correspond to an active uniform variable in program or name starts with the reserved prefix \"gl_\"", name))
        };
        uniform.set(&self.gl, location);
    }
}

impl Drop for Program {
    fn drop(&mut self) {
        unsafe {
            self.gl.delete_program(self.id);
        }
    }
}

pub struct Shader {
    gl: GL,
    id: c_uint,
}

impl Shader {
    pub fn from_source(
        gl: &GL,
        source: &str,
        kind: c_uint,
    ) -> Result<Shader, String> {
        let id = shader_from_source(gl, source, kind)?;
        Ok(Shader { gl: gl.clone(), id })
    }

    pub fn from_vert_source(gl: &GL, source: &str) -> Result<Shader, String> {
        Shader::from_source(gl, source, glow::VERTEX_SHADER)
    }

    pub fn from_frag_source(gl: &GL, source: &str) -> Result<Shader, String> {
        Shader::from_source(gl, source, glow::FRAGMENT_SHADER)
    }

    pub fn id(&self) -> c_uint {
        self.id
    }
}

impl Drop for Shader {
    fn drop(&mut self) {
        unsafe {
            self.gl.delete_shader(self.id);
        }
    }
}

fn shader_from_source(
    gl: &GL,
    source: &str,
    kind: c_uint,
) -> Result<c_uint, String> {
    let id = unsafe { gl.create_shader(kind).expect("Cannot create program") };
    unsafe {
        gl.shader_source(id, source);
        gl.compile_shader(id);
    }
    unsafe {
        if !gl.get_shader_compile_status(id) {
            let error = gl.get_shader_info_log(id);
            return Err(error);
        }
    }
    Ok(id)
}
