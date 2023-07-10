use gl_dstruct::gl;
use crate::{Result, err};


fn png_to_gl_color_format(format: png::ColorType) -> Result<i32> {
    match format {
        png::ColorType::Grayscale => Ok(gl::RED as i32),
        png::ColorType::Rgb => Ok(gl::RGB as i32),
        png::ColorType::GrayscaleAlpha => Ok(gl::RG as i32),
        png::ColorType::Rgba => Ok(gl::RGBA as i32),
        _ => Err(err::type_convert(format!("Color format not supported: '{:?}'", format)))
    }
}


pub struct Texture {
    gl: gl_dstruct::Gl,
    id: u32,
    unit: u32
}

impl Texture {
    pub fn from_file(gl: &gl_dstruct::Gl, path: &str, unit: gl::types::GLuint) -> Result<Self> {
        let decoder = png::Decoder::new(std::fs::File::open(path).map_err(|_|
            err::file_missing("File not found", path.to_string())
        )?);
        let mut reader = decoder.read_info().map_err(|e| err::new(e))?;

        let width = reader.info().width;
        let height = reader.info().height;
        let mut buffer = vec![0; reader.output_buffer_size()];

        reader.next_frame(&mut buffer).map_err(|e| err::new(e))?;

        let mut id = 0;
        unsafe {
            gl.GenTextures(1, &mut id);
            gl.BindTexture(gl::TEXTURE_2D, id);
            gl.TexImage2D(
                gl::TEXTURE_2D,
                0,
                png_to_gl_color_format(reader.info().color_type)?,
                width as i32,
                height as i32,
                0,
                gl::RGBA,
                gl::UNSIGNED_BYTE,
                buffer.as_ptr() as *const gl::types::GLvoid
            );
            gl.BindTextureUnit(unit, id)
        }
        Ok(Self { gl: gl.clone(), id, unit })
    }

    pub fn set_unit(&mut self, gl: &gl_dstruct::Gl, unit: gl::types::GLuint) {
        self.unit = unit;
        unsafe { gl.BindTextureUnit(unit, self.id) }
    }

    pub fn get_id(&self) -> gl::types::GLuint {
        self.id
    }

    pub fn get_unit(&self) -> gl::types::GLuint {
        self.unit
    }

    pub fn parameters(gl: &gl_dstruct::Gl) {
        unsafe {
            gl.TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_S, gl::REPEAT as i32);
            gl.TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_T, gl::REPEAT as i32);
            gl.TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MIN_FILTER, gl::LINEAR as i32);
            gl.TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MAG_FILTER, gl::LINEAR as i32);
        }
    }
}

impl Drop for Texture {
    fn drop(&mut self) {
        unsafe { self.gl.DeleteTextures(1, &self.id) }
    }
}
