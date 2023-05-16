use crate::{Vector2, Color};


#[repr(C, packed)]
pub struct Vertex {
    pub pos: Vector2,
    pub color: Color
}

impl Vertex {
    pub fn new<P: Into<Vector2>, C: Into<Color>>(pos: P, color: C) -> Self {
        Self { pos: pos.into(), color: color.into() }
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
        }
    }
}

impl<P: Into<Vector2>, C: Into<Color>> From<(P, C)> for Vertex {
    fn from(value: (P, C)) -> Self {
        Self { pos: value.0.into(), color: value.1.into() }
    }
}


