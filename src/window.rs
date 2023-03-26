use crate::{err, Event, RenderInstance};


pub struct Window {
    sdl_win: sdl2::video::Window,
    sdl_event_pump: sdl2::EventPump,
    pub gl: gl::Gl,
    open: bool
}

impl Window {
    pub fn new(title: &str, width: u32, height: u32) -> Result<Self, err::Error> {
        let sdl = sdl2::init().map_err(|e| err::new(&e))?;
        let video_subsystem = sdl.video().map_err(|e| err::new(&e))?;
        let gl_attr = video_subsystem.gl_attr();
        gl_attr.set_context_profile(sdl2::video::GLProfile::Core);
        gl_attr.set_context_version(4, 5);

        let window = video_subsystem.window(title, width, height).opengl().build()
            .map_err(|e| err::new(&e.to_string()))?;
        let event_pump = sdl.event_pump().map_err(|e| err::new(e))?;

        let _gl_ctx = window.gl_create_context().map_err(|e| err::new(&e))?;
        let gl = gl::load_with(|s| video_subsystem.gl_get_proc_address(s) as *const std::os::raw::c_void);
        
        unsafe {
            gl.Viewport(0, 0,
                i32::try_from(width).or(Err(err::new("Window width too large")))?,
                i32::try_from(height).or(Err(err::new("Window height too large")))?
            );
        }

        Ok(Self {
            sdl_win: window,
            sdl_event_pump: event_pump,
            gl,
            open: true
        })
    }
    
    // TODO: Custom color type
    pub fn clear(&self, r: f32, g: f32, b: f32, a: f32) {
        unsafe { self.gl.ClearColor(r, g, b, a) }
        unsafe { self.gl.Clear(gl::COLOR_BUFFER_BIT) }
    }

    pub fn get_events(&mut self) -> Vec<Event> {
        let mut events = Vec::new();
        for event in self.sdl_event_pump.poll_iter() {
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

    pub fn render(&self, render_instance: &impl RenderInstance) {
        render_instance.render_instance(&self.gl);
    }

    pub fn update(&self) {
        self.sdl_win.gl_swap_window();
    }

    pub fn is_open(&self) -> bool {
        self.open
    }
 
    pub fn close(&mut self) {
        self.open = false;
    }
}
