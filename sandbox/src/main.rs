

fn main() -> visim::Result<()> {
    let mut window = visim::Window::new("Hello world", 1280, 720)?;
    let mut x = 0.0;
        
    while window.is_open() {
        window.get_events();
        window.renderer.clear((0.5, 0.2, 0.7, 1.0));
        
        x += 0.001;
        window.renderer.polygon(vec![
            ((-1.0, 0.0), (1.0, 0.0, 0.0)),
            ((-0.5, 0.5), (0.0, 1.0, 0.0)),
            ((0.0, 0.0), (0.0, 0.0, 1.0))
        ]);
        window.renderer.polygon(vec![
            ((0.0 + x, 0.0), (1.0, 0.0, 0.0)),
            ((0.5 + x, 0.5), (0.0, 1.0, 0.0)),
            ((1.0 + x, 0.0), (0.0, 0.0, 1.0))
        ]);

        window.update();
    }

    Ok(())
}
