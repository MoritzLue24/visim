#[allow(dead_code)]
pub mod err;


pub fn test() -> Result<(), err::Error> {
    let sdl = err::map_str(sdl2::init(), err::Kind::Other)?;
    let video_subsystem = err::map_str(sdl.video(), err::Kind::Other)?;
    let gl_attr = video_subsystem.gl_attr();
    gl_attr.set_context_profile(sdl2::video::GLProfile::Core);
    gl_attr.set_context_version(4, 5);

    let window = err::map_str(video_subsystem.window("Title", 2_147_483_646, 600).opengl().build(), err::Kind::Other)?;
    
    let _gl_ctx = err::map_str(window.gl_create_context(), err::Kind::Other)?;
    gl::load_with(|s| video_subsystem.gl_get_proc_address(s) as *const std::os::raw::c_void);

    unsafe {
        gl::Viewport(0, 0,
            i32::try_from(window.size().0).or(Err(err::new("Window width too large")))?,
            i32::try_from(window.size().1).or(Err(err::new("Window height too large")))?
        );
        gl::ClearColor(0.3, 0.3, 0.5, 1.);
    }

    let mut event_pump = err::map_str(sdl.event_pump(), err::Kind::Other)?;
    'main: loop {
        while let Some(event) = event_pump.poll_event() {
            match event {
                sdl2::event::Event::Quit { .. } => break 'main,
                _ => ()
            }
        }

        unsafe { gl::Clear(gl::COLOR_BUFFER_BIT) }
        window.gl_swap_window();
    }

    Ok(())
}
