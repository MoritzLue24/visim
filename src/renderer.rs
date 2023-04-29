use crate::{render::{buffer, VertexArray}, Result, Program, err, Vertex, Color};


pub struct Renderer {
    _sdl_gl_ctx: sdl2::video::GLContext,
    gl: gl::Gl,
    program: Program,
    pub vbo: buffer::Array<Vertex>,
    ibo: buffer::ElementArray<u32>,
    vao: VertexArray,
    last_ibo_i: i32
}

impl Renderer {
    pub fn new(window: &sdl2::video::Window) -> Result<Self> {
        let gl_ctx = window.gl_create_context().map_err(|e| err::new(e))?;
        let gl = gl::load_with(|s| window.subsystem().gl_get_proc_address(s) as _);

        let program = Program::default(&gl)?;
        let vbo = buffer::Array::new(&gl, buffer::DrawUsage::Dynamic);
        let ibo = buffer::ElementArray::new(&gl, buffer::DrawUsage::Dynamic);
        let vao = VertexArray::new(&gl);

        vao.bind();
        Vertex::attrib_pointers(&gl);
        vao.unbind();

        let win_size = window.size();
        unsafe {
            gl.Viewport(0, 0,
                i32::try_from(win_size.0).or(Err(err::new("Window width too large")))?,
                i32::try_from(win_size.1).or(Err(err::new("Window height too large")))?
            );
        };

        Ok(Self { 
            _sdl_gl_ctx: gl_ctx,
            gl,
            program,
            vbo,
            ibo,
            vao,
            last_ibo_i: 0
        })
    }

    pub fn render(&self) {
        self.program.bind();
        self.vao.bind();
        unsafe {self.gl.DrawElements(gl::TRIANGLES, self.ibo.len(), gl::UNSIGNED_INT, 0 as *const std::ffi::c_void) }
    }

    pub fn clear<C: Into<Color>>(&self, color: C) {
        self.vbo.write_data(&[]);
        self.ibo.write_data(&[]);
    
        let col: Color = color.into();

        unsafe {
            self.gl.ClearColor(col.r, col.g, col.b, col.a);
            self.gl.Clear(gl::COLOR_BUFFER_BIT);
        }
    }

    pub fn polygon<V: Into<Vertex>>(&self, vertices: Vec<V>) {
        let mut data = Vec::<Vertex>::new();
        for vertex in vertices {
            data.push(vertex.into());
        }

        self.vbo.write_data(&data);
        self.ibo.write_data(&[0, 1, 2]);
    }
}
