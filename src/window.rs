use crate::{err, Event, Result, Renderer};


pub struct Window {
    open: bool,
    pub renderer: Renderer,

    // Declared last to drop last 
    // (to prevent gl error 1282 on glDeleteProgram)
    sdl_win: sdl2::video::Window,
    sdl_event_pump: sdl2::EventPump,
}

impl Window {
    pub fn new(title: &str, width: u32, height: u32) -> Result<Self> {
        let sdl = sdl2::init().map_err(|e| err::new(e))?;
        let video_subsystem = sdl.video().map_err(|e| err::new(e))?;

        let gl_attr = video_subsystem.gl_attr();
        gl_attr.set_context_profile(sdl2::video::GLProfile::Core);
        gl_attr.set_context_version(4, 5);

        let window = video_subsystem.window(title, width, height).opengl().build()
            .map_err(|e| err::new(&e.to_string()))?;
        let event_pump = sdl.event_pump().map_err(|e| err::new(e))?;
        let renderer = Renderer::new(&window)?;

        Ok(Self {
            open: true,
            renderer,
            sdl_win: window,
            sdl_event_pump: event_pump
        })
    }

    pub fn is_open(&self) -> bool {
        self.open
    }
 
    pub fn close(&mut self) {
        self.open = false;
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

    pub fn update(&self) {
        self.renderer.render();
        self.sdl_win.gl_swap_window();
    }
}
