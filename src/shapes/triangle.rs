use crate::{render::{Program, ArrayBuffer, VertexArray, Vertex}, RenderInstance, Result, Window};


pub struct Triangle {
    _vbo: ArrayBuffer,
    vao: VertexArray,
    gl: gl::Gl,
    program: Program
}

impl Triangle {
    pub fn new(window: &Window) -> Result<Self> {
        let gl = window.get_gl();

        let vertices: Vec<Vertex> = vec![
            Vertex { pos: (0.5, -0.5, 0.), color: (0.5, 1., 1., 1.) },  // Bottom right
            Vertex { pos: (-0.5, -0.5, 0.), color: (1., 0.5, 1., 1.) }, // Bottom left
            Vertex { pos: (0.0, 0.5, 0.), color: (1., 1., 0.5, 1.) }    // Top
        ];
        
        let vbo = ArrayBuffer::new(&gl);
        vbo.bind();
        vbo.static_draw_data(&vertices);

        let vao = VertexArray::new(&gl);
        vao.bind();
        Vertex::attrib_pointers(&gl);
        
        vbo.unbind();
        vao.unbind();

        Ok(Self { _vbo: vbo, vao, gl, program: window.get_program() })
    }

    pub fn set_program(&mut self, program: Program) {
        self.program = program;
    }

    pub fn render(&self) {
        self.render_instance()
    }
}

impl RenderInstance for Triangle {
    fn render_instance(&self) {
        self.program.bind();
        self.vao.bind();
        unsafe { self.gl.DrawArrays(gl::TRIANGLES, 0, 3) }
    }
}
