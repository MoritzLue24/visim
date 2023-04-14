use crate::{render::{Program, ArrayBuffer, VertexArray, Vertex}, RenderInstance, Window};


pub struct Polygon {
    _vbo: ArrayBuffer,
    vao: VertexArray,
    gl: gl::Gl,
    program: Program,
    pub fill: bool
}

impl Polygon {
    pub fn new<V: Into<Vertex>>(window: &Window, vertices: Vec<V>) -> Self {
        let gl = window.get_gl();
        
        let mut data = Vec::<Vertex>::new();
        for vertex in vertices {
            data.push(vertex.into())
        }

        let vbo = ArrayBuffer::new(&gl);
        vbo.bind();
        vbo.static_draw_data(&data);

        let vao = VertexArray::new(&gl);
        vao.bind();
        Vertex::attrib_pointers(&gl);
        
        vbo.unbind();
        vao.unbind();

        Self {
            _vbo: vbo,
            vao,
            gl,
            program: window.get_program(),
            fill: true
        }
    }

    pub fn set_program(&mut self, program: Program) {
        self.program = program;
    }

    pub fn render(&self) {
        self.render_instance()
    }
}

impl RenderInstance for Polygon {
    fn render_instance(&self) {
        self.program.bind();
        self.vao.bind();
        
        if self.fill {
            unsafe { self.gl.DrawArrays(gl::TRIANGLE_FAN, 0, 3) }
        } 
        else {
            unsafe { self.gl.DrawArrays(gl::LINE_LOOP, 0, 3) }
        }
    }
}
