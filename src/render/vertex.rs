use crate::{Vector2, Color};
use gl_dstruct::gl;


/// The attribute tex_i = -1 means the Vertex has no texture.
/// As a result, the tex_coords are irrelevant if tex_i = -1.
#[repr(C, packed)]
#[derive(Clone, Copy)]
pub struct Vertex {
    pub pos: Vector2,
    pub color: Color,
    pub tex_i: i32,
    pub tex_coords: Vector2
}

impl Vertex {
    /// The attribute tex_i = -1 means the Vertex has no texture.
    /// As a result, the tex_coords are irrelevant if tex_i = -1.
    pub fn new<V: Into<Vector2>, C: Into<Color>>(pos: V, color: C, tex_i: i32, tex_coords: V) -> Self {
        Self { pos: pos.into(), color: color.into(), tex_i, tex_coords: tex_coords.into() }
    }

    /// Returns a vertex containing a given color.
    /// Therefore the tex_i = -1.
    pub fn col<V: Into<Vector2>, C: Into<Color>>(pos: V, color: C) -> Self {
        Self { pos: pos.into(), color: color.into(), tex_i: -1, tex_coords: (0., 0.).into() }
    }

    pub fn tex<V: Into<Vector2>>(pos: V, tex_i: i32, tex_coords: V) -> Self {
        Self { pos: pos.into(), color: (0., 0., 0.).into(), tex_i, tex_coords: tex_coords.into() }
    }

    pub fn attrib_pointers(gl: &gl_dstruct::Gl, vao_id: gl::types::GLuint) {
        unsafe {
            // Position
            gl.EnableVertexArrayAttrib(vao_id, 0);
            // gl.EnableVertexAttribArray(0);
            gl.VertexAttribPointer(
                0,               // Index of attribute
                2,               // Number of components per attribute
                gl::FLOAT,       // Data type
                gl::FALSE,       // Normalized (int to float conversion)
                (std::mem::size_of::<Self>()) as gl::types::GLint,  // Byte offset between consecutive attributes
                std::ptr::null() // Offset of first component
            );

            // Color 
            gl.EnableVertexArrayAttrib(vao_id, 1);
            // gl.EnableVertexAttribArray(1);
            gl.VertexAttribPointer(
                1,
                4,
                gl::FLOAT,
                gl::FALSE,
                (std::mem::size_of::<Self>()) as gl::types::GLint,
                (std::mem::size_of::<Vector2>()) as *const gl::types::GLvoid
            );

            // Texture index 
            gl.EnableVertexArrayAttrib(vao_id, 2);
            // gl.EnableVertexAttribArray(2);
            // gl.VertexAttribPointer(
            //     2,
            //     1,
            //     gl::INT,
            //     gl::FALSE,
            //     (std::mem::size_of::<Self>()) as gl::types::GLint,
            //     (
            //         std::mem::size_of::<Vector2>() + 
            //         std::mem::size_of::<Color>()
            //     ) as *const gl::types::GLvoid
            // );
            gl.VertexAttribIPointer(
                2,
                1,
                gl::INT,
                (std::mem::size_of::<Self>()) as gl::types::GLint,
                (
                    std::mem::size_of::<Vector2>() + 
                    std::mem::size_of::<Color>()
                ) as *const gl::types::GLvoid
            );

            // Texture coords 
            gl.EnableVertexArrayAttrib(vao_id, 3);
            // gl.EnableVertexAttribArray(3);
            gl.VertexAttribPointer(
                3,
                2,
                gl::FLOAT,
                gl::FALSE,
                (std::mem::size_of::<Self>()) as gl::types::GLint,
                (
                    std::mem::size_of::<Vector2>() + 
                    std::mem::size_of::<Color>() +
                    std::mem::size_of::<i32>()
                ) as *const gl::types::GLvoid
            );
        }
    }
}

impl<V: Into<Vector2>, C: Into<Color>> From<(V, C)> for Vertex {
    fn from(value: (V, C)) -> Self {
        Self::col(value.0.into(), value.1.into())
    }
}

impl<V: Into<Vector2>> From<(V, i32, V)> for Vertex {
    fn from(value: (V, i32, V)) -> Self {
        Self::tex(value.0.into(), value.1, value.2.into())
    }
}
