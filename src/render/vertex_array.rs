
pub struct VertexArray {
    gl: gl::Gl,
    id: gl::types::GLuint
}

impl VertexArray {
    pub fn new(gl: &gl::Gl) -> Self {
        let mut id = 0;
        unsafe { gl.GenVertexArrays(1, &mut id) }
        Self { gl: gl.clone(), id }
    }

    pub fn bind(&self) {
        unsafe { self.gl.BindVertexArray(self.id) }
    }

    pub fn unbind(&self) {
        unsafe { self.gl.BindVertexArray(0) }
    }
}

impl Drop for VertexArray {
    fn drop(&mut self) {
        unsafe { self.gl.DeleteVertexArrays(1, &mut self.id) }
    }
}
