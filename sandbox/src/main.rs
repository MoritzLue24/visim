

fn main() -> visim::Result<()> {
    let mut window = visim::Window::new("Hello world", 1280, 720)?;
    
    while window.is_open() {
        for event in window.get_events() {
            match event {
                visim::Event::Quit { .. } => println!("quit"),
                visim::Event::KeyDown { .. } => println!("{:?}", event),
                _ => ()
            }
        }
    
        window.clear(0.5, 0.2, 0.7, 1.0);
        window.update();
    }

    Ok(())
}
