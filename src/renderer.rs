use crate::{render::{buffer, VertexArray, Texture}, Result, Program, err, Vertex, Color};
use gl_dstruct::gl;


pub struct Renderer {
    pub vertices_len: u32,
    max_vertices: isize,
    textures: [i32; 1],

    // Todo make gl private
    pub gl: gl_dstruct::Gl,
    pub program: Program,
    pub vao: VertexArray,
    pub vbo: buffer::Array<Vertex>,
    pub ibo: buffer::ElementArray<u32>,
    
    // Declared last to drop last 
    // (to prevent gl error 1282 on glDelete)
    _sdl_gl_ctx: sdl2::video::GLContext
}

impl Renderer {
    pub fn new(window: &sdl2::video::Window, max_vertices: isize) -> Result<Self> {
        let gl_ctx = window.gl_create_context().map_err(|e| err::new(e))?;
        let gl = gl_dstruct::load_with(|s| window.subsystem().gl_get_proc_address(s) as _);

        unsafe {
            gl.Enable(gl::DEBUG_OUTPUT);
            gl.DebugMessageCallback(Some(err::gl_debug_callback), std::ptr::null());
        }

        let program = Program::default(&gl)?;
        let vao = VertexArray::new(&gl);
        let mut vbo = buffer::Array::new(&gl, buffer::DrawUsage::Dynamic);
        let mut ibo = buffer::ElementArray::new(&gl, buffer::DrawUsage::Dynamic);
        
        vao.bind();
        vbo.bind();
        ibo.bind();
        Vertex::attrib_pointers(&gl, vao.get_id());

        vbo.allocate_data(std::mem::size_of::<Vertex>() as isize * max_vertices);
        ibo.allocate_data(std::mem::size_of::<u32>() as isize * max_vertices);
        
        Texture::parameters(&gl);

        let win_size = window.size();
        unsafe {
            gl.Viewport(0, 0,
                i32::try_from(win_size.0).or(Err(err::new("Window width too large")))?,
                i32::try_from(win_size.1).or(Err(err::new("Window height too large")))?
            );
        };

        Ok(Self { 
            vertices_len: 0, max_vertices, textures: [-1; 1],
            gl, program, vbo, ibo, vao,
            _sdl_gl_ctx: gl_ctx,
        })
    }

    pub fn render(&self) {
        self.program.bind();
        self.vao.bind();
        // self.program.set_uniform_iv(&self.gl, "u_Textures", &self.textures);
        unsafe { self.gl.DrawElements(gl::TRIANGLES, self.vertices_len as i32, gl::UNSIGNED_INT, 0 as *const std::ffi::c_void) }
    }

    pub fn clear<C: Into<Color>>(&mut self, color: C) {
        self.vertices_len = 0;
        self.vbo.allocate_data(std::mem::size_of::<Vertex>() as isize * self.max_vertices);
        self.ibo.allocate_data(std::mem::size_of::<u32>() as isize * self.max_vertices);

        let col: Color = color.into();

        unsafe {
            self.gl.ClearColor(col.r, col.g, col.b, col.a);
            self.gl.Clear(gl::COLOR_BUFFER_BIT);
        }
    }

    pub fn texture<V: Into<Vertex>>(&mut self, vertices: Vec<V>, texture: &Texture, index: usize) {
        self.polygon(vertices);
        self.textures[index] = texture.get_unit() as i32;
    }

    pub fn polygon<V: Into<Vertex>>(&mut self, vertices: Vec<V>) {
        let data: Vec<Vertex> = vertices.into_iter().map(|v| v.into()).collect();
        self.vbo.append_data(&data);

        // Implement an algorithm that creates indices for
        // triangles based on the vertices.
        // for _ in 0..data.len() {
        //     indices.push(self.last_ibo_i);
        //     self.last_ibo_i += 1;
        // }

        let indices = vec![self.vertices_len, self.vertices_len + 1, self.vertices_len + 2];
        self.ibo.append_data(&indices);
        // TODO: Fix: data.len() is to "small" it can only draw triangles.
        self.vertices_len += data.len() as u32;
    }
}
