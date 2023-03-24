

fn main() -> Result<(), visim::err::Error> {
    let mut window = visim::Window::new("Hello world", 1280, 720)?;
    
    let t = visim::shapes::Triangle::new(&window.gl)?;

    while window.is_open() {
        for event in window.get_events() {
            match event {
                visim::Event::Quit { .. } => println!("quit"),
                visim::Event::KeyDown { .. } => println!("{:?}", event),
                _ => ()
            }
        }
        
        window.render(&t);
    }

    Ok(())
}
