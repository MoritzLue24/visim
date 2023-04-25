use crate::{err, Event, Result, render::Program};

pub type Font<'rwops> = sdl2::ttf::Font<'static, 'rwops>;


pub struct Window {
    _sdl_gl_ctx: sdl2::video::GLContext,
    sdl_win: sdl2::video::Window,
    sdl_event_pump: sdl2::EventPump,
    sdl_ttf: &'static sdl2::ttf::Sdl2TtfContext,
    font: Font<'static>,
    gl: gl::Gl,
    program: Program,
    open: bool
}

impl Window {
    pub fn new(title: &str, width: u32, height: u32) -> Result<Self> {
        let sdl = sdl2::init().map_err(|e| err::new(e))?;
        let video_subsystem = sdl.video().map_err(|e| err::new(e))?;
        let ttf = sdl2::ttf::init().map_err(|e| err::new(e))?;

        let gl_attr = video_subsystem.gl_attr();
        gl_attr.set_context_profile(sdl2::video::GLProfile::Core);
        gl_attr.set_context_version(4, 5);

        let window = video_subsystem.window(title, width, height).opengl().build()
            .map_err(|e| err::new(&e.to_string()))?;
        let event_pump = sdl.event_pump().map_err(|e| err::new(e))?;
        let font = ttf.load_font("font_path", 128).map_err(|e| err::new(e))?;

        let gl_ctx = window.gl_create_context().map_err(|e| err::new(e))?;
        let gl = gl::load_with(|s| video_subsystem.gl_get_proc_address(s) as _);
        let program = Program::default(&gl)?;

        unsafe {
            gl.Viewport(0, 0,
                i32::try_from(width).or(Err(err::new("Window width too large")))?,
                i32::try_from(height).or(Err(err::new("Window height too large")))?
            );
        }

        Ok(Self {
            _sdl_gl_ctx: gl_ctx,
            sdl_win: window,
            sdl_event_pump: event_pump,
            sdl_ttf: &ttf,
            font,
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
        self.sdl_win.gl_swap_window();
    }

    pub fn is_open(&self) -> bool {
        self.open
    }
 
    pub fn close(&mut self) {
        self.open = false;
    }
}
