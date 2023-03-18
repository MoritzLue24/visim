#[allow(dead_code)]
pub mod err;
mod render;
// mod shader;
// mod program;

use std::ffi::CString;
// use program::Program;
// use shader::Shader;
use render::{Vertex, Shader, Program};


pub fn test() -> Result<(), err::Error> {
    let sdl = sdl2::init().map_err(|e| err::new(&e))?;
    let video_subsystem = sdl.video().map_err(|e| err::new(&e))?;
    let gl_attr = video_subsystem.gl_attr();
    gl_attr.set_context_profile(sdl2::video::GLProfile::Core);
    gl_attr.set_context_version(4, 5);

    let window = video_subsystem.window("Title", 1080, 720).opengl().build()
        .map_err(|e| err::new(&e.to_string()))?;
    
    let _gl_ctx = window.gl_create_context().map_err(|e| err::new(&e))?;
    let gl = gl::load_with(|s| video_subsystem.gl_get_proc_address(s) as *const std::os::raw::c_void);
    
    let vert_shader = Shader::from_source(&gl,
        &CString::new(include_str!("shaders/triangle.vert")).map_err(|e| err::new(e))?,
        gl::VERTEX_SHADER
    )?;

    let frag_shader = Shader::from_source(&gl,
        &CString::new(include_str!("shaders/triangle.frag")).map_err(|e| err::new(e))?,
        gl::FRAGMENT_SHADER
    )?;

    let shader_program = Program::from_shaders(&gl, &[vert_shader, frag_shader])?;
    shader_program.set_used();

    let vertices: Vec<Vertex> = vec![
        // Positions     // Colors
        Vertex { pos: (0.5, -0.5, 0.), color: (0.5, 1., 1., 1.) },  // Bottom right
        Vertex { pos: (-0.5, -0.5, 0.), color: (1., 0.5, 1., 1.) }, // Bottom left
        Vertex { pos: (0.0, 0.5, 0.), color: (1., 1., 0.5, 1.) }    // Top
    ];

    let mut vbo = 0;
    unsafe { gl.GenBuffers(1, &mut vbo) }
    unsafe {
        gl.BindBuffer(gl::ARRAY_BUFFER, vbo);
        gl.BufferData(
            gl::ARRAY_BUFFER,
            (vertices.len() * std::mem::size_of::<Vertex>()) as gl::types::GLsizeiptr,
            vertices.as_ptr() as *const gl::types::GLvoid,
            gl::STATIC_DRAW
        );
        gl.BindBuffer(gl::ARRAY_BUFFER, 0);
    }

    let mut vao = 0;
    unsafe { gl.GenVertexArrays(1, &mut vao) }
    unsafe {
        gl.BindVertexArray(vao);
        gl.BindBuffer(gl::ARRAY_BUFFER, vbo);
        
        Vertex::attrib_pointers(&gl);
                
        gl.BindBuffer(gl::ARRAY_BUFFER, 0);
        gl.BindVertexArray(0);
    }

    unsafe {
        gl.Viewport(0, 0,
            i32::try_from(window.size().0).or(Err(err::new("Window width too large")))?,
            i32::try_from(window.size().1).or(Err(err::new("Window height too large")))?
        );
        gl.ClearColor(0.3, 0.3, 0.5, 1.);
    }

    let mut event_pump = sdl.event_pump().map_err(|e| err::new(e))?;
    'main: loop {
        while let Some(event) = event_pump.poll_event() {
            match event {
                sdl2::event::Event::Quit { .. } => break 'main,
                _ => ()
            }
        }

        unsafe {
            gl.Clear(gl::COLOR_BUFFER_BIT);
            gl.BindVertexArray(vao);
            gl.DrawArrays(gl::TRIANGLE_STRIP, 0, 3);
        }

        window.gl_swap_window();
    }

    Ok(())
}
