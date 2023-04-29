use crate::{render::{buffer, VertexArray}, Result, Program, err, Vertex, Color};


pub struct Renderer {
    _sdl_gl_ctx: sdl2::video::GLContext,
    gl: gl::Gl,
    program: Program,
    vbo: buffer::Array,
    vao: VertexArray,
    ibo: buffer::ElementArray
}

impl Renderer {
    pub fn new(window: &sdl2::video::Window) -> Result<Self> {
        let gl_ctx = window.gl_create_context().map_err(|e| err::new(e))?;
        let gl = gl::load_with(|s| window.subsystem().gl_get_proc_address(s) as _);

        Vertex::attrib_pointers(&gl);
        let program = Program::default(&gl)?;
        let vbo = buffer::Array::new(&gl, buffer::DrawUsage::Dynamic);
        let vao = VertexArray::new(&gl);
        let ibo = buffer::ElementArray::new(&gl, buffer::DrawUsage::Dynamic);
        
        let win_size = window.size();
        unsafe {
            gl.Viewport(0, 0,
                i32::try_from(win_size.0).or(Err(err::new("Window width too large")))?,
                i32::try_from(win_size.1).or(Err(err::new("Window height too large")))?
            );
        };

        Ok(Self { _sdl_gl_ctx: gl_ctx, gl, program, vbo, vao, ibo })
    }

    pub fn render(&self) {

    }

    pub fn clear<C: Into<Color>>(&self, color: C) {
        self.vbo.write_data::<Vertex>(&[]);
        self.ibo.write_data::<i32>(&[]);
    
        let col: Color = color.into();

        unsafe {
            self.gl.ClearColor(col.r, col.g, col.b, col.a);
            self.gl.Clear(gl::COLOR_BUFFER_BIT);
        }
    }
}
