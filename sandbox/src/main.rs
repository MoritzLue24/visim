

fn main() -> Result<(), visim::err::Error> {
    let mut window = visim::Window::new("Hello world", 1280, 720)?;
    
    while window.is_open() {
        for event in window.get_events() {
            match event {
                visim::Event::KeyDown { timestamp: _, window_id: _, keycode, .. } => println!("{:?}", keycode),
                _ => ()
            }
        }

        window.display();
    }

    Ok(())
}
