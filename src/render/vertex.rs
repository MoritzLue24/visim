
#[derive(Clone, Copy, Debug)]
#[repr(C, packed)]
pub struct Vertex {
    pub pos: (f32, f32, f32),
    pub color: (f32, f32, f32, f32)
}

impl Vertex {
    pub fn attrib_pointers(gl: &gl::Gl) {
        unsafe {
            // Position
            gl.EnableVertexAttribArray(0);
            gl.VertexAttribPointer(
                0,               // Index of attribute
                3,               // Number of components per attribute
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
                (3 * std::mem::size_of::<f32>()) as *const gl::types::GLvoid    // Offset of first component
            );
        }
    }
}
