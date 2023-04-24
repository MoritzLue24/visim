use glfw::Context;

use crate::{err, Event, Result, render::Program};


pub struct Window {
    glfw: glfw::Glfw,
    glfw_win: glfw::Window,
    glfw_events: sdl2::EventPump,
    gl: gl::Gl,
    program: Program,
    open: bool
}

impl Window {
    pub fn new(title: &str, width: u32, height: u32) -> Result<Self> {
        // gl_attr.set_context_version(4, 5);
        let glfw = glfw::init(glfw::FAIL_ON_ERRORS).map_err(|e| err::new(e))?;
        let (window, events) = glfw.create_window(width, height, title, glfw::WindowMode::Windowed).ok_or(err::new("Window could not be initialized"))?;
        window.make_current();

        let gl = gl::load_with(|s| video_subsystem.gl_get_proc_address(s) as _);
        let program = Program::default(&gl)?;

        unsafe {
            gl.Viewport(0, 0,
                i32::try_from(width).or(Err(err::new("Window width too large")))?,
                i32::try_from(height).or(Err(err::new("Window height too large")))?
            );
        }

        Ok(Self {
            glfw,
            glfw_win: window,
            glfw_events: events,
            gl,
            open: true,
            program
        })
    }
    
    pub fn get_gl(&self) -> gl::Gl {
        self.gl.clone()
    }

    pub fn get_program(&self) -> Program {
        self.program.clone()
    }

    pub fn clear(&self, r: f32, g: f32, b: f32, a: f32) {
        unsafe { self.gl.ClearColor(r, g, b, a) }
        unsafe { self.gl.Clear(gl::COLOR_BUFFER_BIT) }
    }

    pub fn get_events(&mut self) -> Vec<Event> {
        let mut events = Vec::new();
        self.glfw.poll_events();

        for event in glfw::flush_messages(self.glfw_events) {
            match Event::from(event) {
                Ok(e) => { 
                    match e {
                        Event::Quit => self.open = false,
                        _ => ()
                    };
                    events.push(e)
                },
                Err(e) => match e {
                    _ => ()
                },
            }
        }
        events
    }

    pub fn update(&self) {
        self.sdl_win.gl_swap_window();
    }

    pub fn is_open(&self) -> bool {
        self.glfw_win.should_close()
    }
 
    pub fn close(&mut self) {
        self.glfw_win.set_should_close(true);
    }
}
