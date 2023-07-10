

fn main() -> visim::Result<()> {
    let mut window = visim::Window::new("Hello world", 1280, 720)?;
 
    // let v1 = visim::Vertex { pos: (0., 0.5).into(), color: (0., 1., 0.).into(), tex_i: -1, tex_coords: (0., 0.).into() };
    // let v2 = visim::Vertex { pos: (-0.5, -0.5).into(), color: (0., 1., 0.).into(), tex_i: -1, tex_coords: (0., 0.).into() };
    // let v3 = visim::Vertex { pos: (0.5, -0.5).into(), color: (0., 1., 0.).into(), tex_i: -1, tex_coords: (0., 0.).into() };
    let v1 = visim::Vertex { pos: (0., 0.5).into(), color: (1., 0., 0.).into(), tex_i: 0, tex_coords: (0.5, 1.).into() };
    let v2 = visim::Vertex { pos: (-0.5, -0.5).into(), color: (1., 0., 0.).into(), tex_i: 0, tex_coords: (0., 0.).into() };
    let v3 = visim::Vertex { pos: (0.5, -0.5).into(), color: (1., 0., 0.).into(), tex_i: 0, tex_coords: (1., 0.).into() };

    let texture = visim::Texture::from_file(&window.renderer.gl, "res/test.png", 1)?;
    
    window.renderer.program.bind();
    window.renderer.vao.bind();
    window.renderer.program.set_uniform_1i(&window.renderer.gl, "u_Texture", 1);

    while window.is_open() {
        window.get_events();
        window.renderer.clear((0.5, 0.2, 0.7, 1.0));
        
        // window.renderer.polygon(vec![v1, v2, v3]);

        window.renderer.texture(vec![v1, v2, v3], &texture, 0);

        // window.renderer.polygon(vec![
        //     visim::Vertex::tex((0., 0.5), 50, (0., 0.)),
        //     visim::Vertex::tex((-0.5, -0.5), 50, (0., 0.)),
        //     visim::Vertex::tex((0.5, -0.5), 50, (0., 0.)),
        // ]);

        window.update();
    }

    Ok(())
}
