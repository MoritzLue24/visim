use crate::{render::{ArrayBuffer, VertexArray}, Program, Vertex, Window};



pub struct Line {
    _vbo: ArrayBuffer,
    vao: VertexArray,
    gl: gl::Gl,
    program: Program,
}

impl Line {
    pub fn new<V: Into<Vertex>>(window: &Window, a: V, b: V) -> Self {
        let gl = window.get_gl();

        let vbo = ArrayBuffer::new(&gl);
        vbo.bind();
        
        let data: Vec<Vertex> = vec![a.into(), b.into()];
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
            program: window.get_program()
        }
    }
}