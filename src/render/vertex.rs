use crate::{Vector2, Color};


#[repr(C, packed)]
pub struct Vertex {
    pub pos: Vector2,
    pub color: Color,
    pub tex_id: f32,
    pub tex_coords: Vector2,
}

impl Vertex {
    pub fn new<V: Into<Vector2>, C: Into<Color>>(pos: V, color: C, tex_id: f32, tex_coords: V) -> Self {
        Self { pos: pos.into(), color: color.into(), tex_id, tex_coords: tex_coords.into() }
    }

    pub fn attrib_pointers(gl: &gl::Gl, vao_id: gl::types::GLuint) {
        unsafe {
            // Position
            gl.EnableVertexArrayAttrib(vao_id, 0);
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
            gl.VertexAttribPointer(
                1,               // Index of attribute
                4,               // Number of components per attribute
                gl::FLOAT,       // Data type
                gl::FALSE,       // Normalized (int to float conversion)
                (std::mem::size_of::<Self>()) as gl::types::GLint,              // Byte offset between consecutive attributes
                (std::mem::size_of::<Vector2>()) as *const gl::types::GLvoid    // Offset of first component
            );

            // Texture id 
            gl.EnableVertexArrayAttrib(vao_id, 2);
            gl.VertexAttribPointer(
                2,
                1,
                gl::FLOAT,
                gl::FALSE,
                std::mem::size_of::<Self>() as gl::types::GLint,
                (std::mem::size_of::<Vector2>() + std::mem::size_of::<Color>()) as *const gl::types::GLvoid
            );

            // Texture coordinates
            gl.EnableVertexArrayAttrib(vao_id, 3);
            gl.VertexAttribPointer(
                3,
                2,
                gl::FLOAT,
                gl::FALSE,
                std::mem::size_of::<Self>() as gl::types::GLint,
                (
                    std::mem::size_of::<Vector2>() + 
                    std::mem::size_of::<Color>() +
                    std::mem::size_of::<f32>()
                ) as *const gl::types::GLvoid
            );
        }
    }
}

impl<V: Into<Vector2>, C: Into<Color>> From<(V, C, f32, V)> for Vertex {
    fn from(value: (V, C, f32, V)) -> Self {
        Self { pos: value.0.into(), color: value.1.into(), tex_id: value.2, tex_coords: value.3.into() }
    }
}


