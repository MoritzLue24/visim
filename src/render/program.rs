use std::ffi::CString;
use crate::{render::Shader, err, Result, ShaderType};


#[derive(Clone)]
pub struct Program {
    gl: gl::Gl,
    id: gl::types::GLuint
}

impl Program {
    pub fn default(gl: &gl::Gl) -> Result<Self> {
        Self::from_shaders(gl, &[
            Shader::from_source(gl, include_str!("../shaders/default.vert"), ShaderType::VertexShader)?,
            Shader::from_source(gl, include_str!("../shaders/default.frag"), ShaderType::FragmentShader)?
        ])
    }

    pub fn from_shaders(gl: &gl::Gl, shaders: &[Shader]) -> Result<Self> {
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
}

impl Drop for Program {
    fn drop(&mut self) {
        unsafe { self.gl.DeleteProgram(self.id) }
    }
}
