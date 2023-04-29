use crate::{Vector2, Color};


#[derive(Clone, Copy, Debug)]
#[repr(C, packed)]
pub struct Vertex {
    pub pos: Vector2,
    pub color: Color
}

impl Vertex {
    pub fn new<P: Into<Vector2>, C: Into<Color>>(pos: P, color: C) -> Self {
        Self { pos: pos.into(), color: color.into() }
    }

    pub fn attrib_pointers(gl: &gl::Gl) {
        unsafe {
            // Position
            gl.EnableVertexAttribArray(0);
            gl.VertexAttribPointer(
                0,               // Index of attribute
                2,               // Number of components per attribute
                gl::FLOAT,       // Data type
                gl::FALSE,       // Normalized (int to float conversion)
                (std::mem::size_of::<Self>()) as gl::types::GLint,  // Byte offset between consecutive attributes
                std::ptr::null() // Offset of first component
            );

            // Color 
            gl.EnableVertexAttribArray(1);
            gl.VertexAttribPointer(
                1,               // Index of attribute
                4,               // Number of components per attribute
                gl::FLOAT,       // Data type
                gl::FALSE,       // Normalized (int to float conversion)
                (std::mem::size_of::<Self>()) as gl::types::GLint,              // Byte offset between consecutive attributes
                (std::mem::size_of::<Vector2>()) as *const gl::types::GLvoid    // Offset of first component
            );

            // Fix gl error 1282
            // (occurs on second glVertexAttribPointer call).
            todo!()
        }
    }
}

impl<P: Into<Vector2>, C: Into<Color>> From<(P, C)> for Vertex {
    fn from(value: (P, C)) -> Self {
        Self { pos: value.0.into(), color: value.1.into() }
    }
}


