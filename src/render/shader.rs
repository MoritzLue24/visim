use std::ffi::CString;
use gl::Gl;
use crate::{err, Result};


pub enum ShaderType {
    VertexShader = 35633,
    TessControlShader = 36488,
    TessEvaluationShader = 36487,
    GeometryShader = 36313,
    FragmentShader = 35632,
    ComputeShader = 37305
}


pub struct Shader {
    gl: Gl,
    id: gl::types::GLuint
}

impl Shader {
    pub fn from_source(gl: &Gl, source: &str, kind: ShaderType) -> Result<Self> {
        let id = unsafe { gl.CreateShader(kind as u32) };
        
        unsafe {
            gl.ShaderSource(id, 1, &CString::new(source).map_err(|e| err::new(e))?.as_ptr(), std::ptr::null());
            gl.CompileShader(id);
        }

        let mut success = 1;
        unsafe { gl.GetShaderiv(id, gl::COMPILE_STATUS, &mut success) }
    
        if success == 0 {
            let mut buf_len = 0;
            unsafe { gl.GetShaderiv(id, gl::INFO_LOG_LENGTH, &mut buf_len) }
        
            let mut buffer: Vec<u8> = Vec::with_capacity(buf_len as usize + 1);
            buffer.extend([b' '].iter().cycle().take(buf_len as usize));
        
            let error = unsafe { CString::from_vec_unchecked(buffer) };
            unsafe { gl.GetShaderInfoLog(
                    id, buf_len, 
                    std::ptr::null_mut(), 
                    error.as_ptr() as *mut gl::types::GLchar
            ) }
        
            return Err(err::parse_shader(&error.to_string_lossy()))
        }

        Ok(Shader { gl: gl.clone(), id })
    }

    pub fn id(&self) -> gl::types::GLuint {
        self.id
    }
}

impl Drop for Shader {
    fn drop(&mut self) {
        unsafe { self.gl.DeleteShader(self.id) }
    }
}
