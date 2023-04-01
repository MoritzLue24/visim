

fn main() -> visim::Result<()> {
    let mut window = visim::Window::new("Hello world", 1280, 720)?;

    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();
    let trimmed = input.trim();
    let x = match trimmed.parse::<f32>() {
        Ok(f) => f,
        Err(..) => panic!()
    };

    let mut t = visim::shapes::Polygon::new(&window, &[visim::Vertex { pos: (x, 0.5, 0.0), color: (1.0, 1.0, 0.0, 1.0) }])?;
    t.fill = true;   

    while window.is_open() {
        for event in window.get_events() {
            match event {
                visim::Event::Quit { .. } => println!("quit"),
                visim::Event::KeyDown { .. } => println!("{:?}", event),
                _ => ()
            }
        }
    
        window.clear(0.5, 0.2, 0.7, 1.0);
        t.render();
        window.update();
    }

    Ok(())
}
