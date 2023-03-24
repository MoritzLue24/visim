use std::ffi::CString;

use crate::{render::{Program, ArrayBuffer, VertexArray, Shader, Vertex}, err, RenderInstance};


pub struct Triangle {
    program: Program,
    _vbo: ArrayBuffer,
    vao: VertexArray
}

impl Triangle {
    pub fn new(gl: &gl::Gl) -> Result<Self, err::Error> {
        let vert_shader = Shader::from_source(&gl,
            &CString::new(include_str!("../shaders/triangle.vert")).map_err(|e| err::new(e))?,
            gl::VERTEX_SHADER
        )?;

        let frag_shader = Shader::from_source(&gl,
            &CString::new(include_str!("../shaders/triangle.frag")).map_err(|e| err::new(e))?,
            gl::FRAGMENT_SHADER
        )?;

        let program = Program::from_shaders(&gl, &[vert_shader, frag_shader])?;
        let vertices: Vec<Vertex> = vec![
            Vertex { pos: (0.5, -0.5, 0.), color: (0.5, 1., 1., 1.) },  // Bottom right
            Vertex { pos: (-0.5, -0.5, 0.), color: (1., 0.5, 1., 1.) }, // Bottom left
            Vertex { pos: (0.0, 0.5, 0.), color: (1., 1., 0.5, 1.) }    // Top
        ];
        
        let vbo = ArrayBuffer::new(gl);
        vbo.bind();
        vbo.static_draw_data(&vertices);

        let vao = VertexArray::new(gl);
        vao.bind();
        Vertex::attrib_pointers(gl);
        
        vbo.unbind();
        vao.unbind();

        Ok(Self { program, _vbo: vbo, vao })
    }
}

impl RenderInstance for Triangle {
    fn render_instance(&self, gl: &gl::Gl) {
        self.program.bind();
        self.vao.bind();

        unsafe {
            gl.DrawArrays(gl::TRIANGLES, 0, 3)
        }
    }
}
