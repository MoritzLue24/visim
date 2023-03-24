pub mod err;
mod render;
mod render_instance;
pub mod shapes;
pub mod event;
mod window;

pub use render_instance::RenderInstance;
pub use event::Event;
pub use window::Window;


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

    unsafe {
        gl.Viewport(0, 0,
            i32::try_from(window.size().0).or(Err(err::new("Window width too large")))?,
            i32::try_from(window.size().1).or(Err(err::new("Window height too large")))?
        );
        gl.ClearColor(0.3, 0.3, 0.5, 1.);
    }

    let triangle = shapes::Triangle::new(&gl)?;

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
        }

        window.gl_swap_window();
    }

    Ok(())
}
