use gl_dstruct::gl;


pub struct VertexArray {
    gl: gl_dstruct::Gl,
    id: gl::types::GLuint
}

impl VertexArray {
    pub fn new(gl: &gl_dstruct::Gl) -> Self {
        let mut id = 0;
        unsafe { gl.GenVertexArrays(1, &mut id) }
        Self { gl: gl.clone(), id }
    }

    pub fn get_id(&self) -> gl::types::GLuint {
        self.id
    }

    pub fn bind(&self) {
        unsafe { self.gl.BindVertexArray(self.id) }
    }
}

impl Drop for VertexArray {
    fn drop(&mut self) {
        unsafe { self.gl.DeleteVertexArrays(1, &mut self.id) }
    }
}
