pub mod err;
pub mod event;
mod vector;
mod color;
mod render;
mod renderer;
mod window;

pub use err::Result;
pub use event::Event;
pub use renderer::Renderer;
pub use window::Window;
pub use vector::Vector2;
pub use color::Color;
pub use render::{Program, ShaderType, Shader, Vertex};


#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn win_loop() -> Result<()> {
        let mut window = Window::new("Hello world", 800, 600)?;
        while window.is_open() {
            for event in window.get_events() {
                match event {
                    _ => ()
                }
            }
            window.clear(0.5, 0.5, 0.7, 1.0);
            window.update();
        }
        Ok(())
    }
}
