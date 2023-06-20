use std::ffi::CString;
use gl_dstruct::gl;
use crate::{render::Shader, err, Result, ShaderType};


#[derive(Clone)]
pub struct Program {
    gl: gl_dstruct::Gl,
    id: gl::types::GLuint
}

impl Program {
    pub fn default(gl: &gl_dstruct::Gl) -> Result<Self> {
        Self::from_shaders(gl, &[
            Shader::from_source(gl, include_str!("../shaders/default.vert"), ShaderType::VertexShader)?,
            Shader::from_source(gl, include_str!("../shaders/default.frag"), ShaderType::FragmentShader)?
        ])
    }

    pub fn from_shaders(gl: &gl_dstruct::Gl, shaders: &[Shader]) -> Result<Self> {
        let id = unsafe { gl.CreateProgram() };
        for shader in shaders {
            unsafe { gl.AttachShader(id, shader.id()) }
        }
        unsafe { gl.LinkProgram(id) }

        let mut success = 1;
        unsafe { gl.GetProgramiv(id, gl::LINK_STATUS, &mut success) }
        
        if success == 0 {
            let mut buf_len = 0;
            unsafe { gl.GetProgramiv(id, gl::INFO_LOG_LENGTH, &mut buf_len) }

            let mut buffer: Vec<u8> = Vec::with_capacity(buf_len as usize + 1);
            buffer.extend([b' '].iter().cycle().take(buf_len as usize));
        
            let error = unsafe { CString::from_vec_unchecked(buffer) };
            unsafe { gl.GetProgramInfoLog(
                    id, buf_len, 
                    std::ptr::null_mut(), 
                    error.as_ptr() as *mut gl::types::GLchar
            ) }

            return Err(err::link_program(&error.to_string_lossy()))
        }

        for shader in shaders {
            unsafe { gl.DetachShader(id, shader.id()) }
        }

        Ok(Program { gl: gl.clone(), id })
    }

    pub fn bind(&self) {
        unsafe { self.gl.UseProgram(self.id) }
    }

    fn get_uniform_loc(&self, gl: &gl_dstruct::Gl, name: &str) -> i32 {
        unsafe { gl.GetUniformLocation(self.id, name.as_ptr() as *const gl::types::GLchar) }
    }


    pub fn set_uniform_1f(&self, gl: &gl_dstruct::Gl, name: &str, v0: f32) {
        let loc = self.get_uniform_loc(gl, name);
        unsafe { gl.Uniform1f(loc, v0) }
    }
    
    pub fn set_uniform_2f(&self, gl: &gl_dstruct::Gl, name: &str, v0: f32, v1: f32) {
        let loc = self.get_uniform_loc(gl, name);
        unsafe { gl.Uniform2f(loc, v0, v1) }
    }

    pub fn set_uniform_3f(&self, gl: &gl_dstruct::Gl, name: &str, v0: f32, v1: f32, v2: f32) {
        let loc = self.get_uniform_loc(gl, name);
        unsafe { gl.Uniform3f(loc, v0, v1, v2) }
    }

    pub fn set_uniform_4f(&self, gl: &gl_dstruct::Gl, name: &str, v0: f32, v1: f32, v2: f32, v3: f32) {
        let loc = self.get_uniform_loc(gl, name);
        unsafe { gl.Uniform4f(loc, v0, v1, v2, v3) }
    }

    pub fn set_uniform_1i(&self, gl: &gl_dstruct::Gl, name: &str, v0: i32) {
        let loc = self.get_uniform_loc(gl, name);
        unsafe { gl.Uniform1i(loc, v0) }
    }

    pub fn set_uniform_2i(&self, gl: &gl_dstruct::Gl, name: &str, v0: i32, v1: i32) {
        let loc = self.get_uniform_loc(gl, name);
        unsafe { gl.Uniform2i(loc, v0, v1) }
    }

    pub fn set_uniform_3i(&self, gl: &gl_dstruct::Gl, name: &str, v0: i32, v1: i32, v2: i32) {
        let loc = self.get_uniform_loc(gl, name);
        unsafe { gl.Uniform3i(loc, v0, v1, v2) }
    }

    pub fn set_uniform_4i(&self, gl: &gl_dstruct::Gl, name: &str, v0: i32, v1: i32, v2: i32, v3: i32) {
        let loc = self.get_uniform_loc(gl, name);
        unsafe { gl.Uniform4i(loc, v0, v1, v2, v3) }
    }

    pub fn set_uniform_1ui(&self, gl: &gl_dstruct::Gl, name: &str, v0: u32) {
        let loc = self.get_uniform_loc(gl, name);
        unsafe { gl.Uniform1ui(loc, v0) }
    }

    pub fn set_uniform_2ui(&self, gl: &gl_dstruct::Gl, name: &str, v0: u32, v1: u32) {
        let loc = self.get_uniform_loc(gl, name);
        unsafe { gl.Uniform2ui(loc, v0, v1) }
    }

    pub fn set_uniform_3ui(&self, gl: &gl_dstruct::Gl, name: &str, v0: u32, v1: u32, v2: u32) {
        let loc = self.get_uniform_loc(gl, name);
        unsafe { gl.Uniform3ui(loc, v0, v1, v2) }
    }

    pub fn set_uniform_4ui(&self, gl: &gl_dstruct::Gl, name: &str, v0: u32, v1: u32, v2: u32, v3: u32) {
        let loc = self.get_uniform_loc(gl, name);
        unsafe { gl.Uniform4ui(loc, v0, v1, v2, v3) }
    }

    pub fn set_uniform_fv(&self, gl: &gl_dstruct::Gl, name: &str, v: &[f32]) {
        let loc = self.get_uniform_loc(gl, name);
        unsafe { gl.Uniform1fv(loc, v.len() as i32, v.as_ptr()) }
    }
    
    pub fn set_uniform_iv(&self, gl: &gl_dstruct::Gl, name: &str, v: &[i32]) {
        let loc = self.get_uniform_loc(gl, name);
        unsafe { gl.Uniform1iv(loc, v.len() as i32, v.as_ptr()) }
    }

    pub fn set_uniform_uiv(&self, gl: &gl_dstruct::Gl, name: &str, v: &[u32]) {
        let loc = self.get_uniform_loc(gl, name);
        unsafe { gl.Uniform1uiv(loc, v.len() as i32, v.as_ptr()) }
    }
}

impl Drop for Program {
    fn drop(&mut self) {
        unsafe { self.gl.DeleteProgram(self.id) }
    }
}
