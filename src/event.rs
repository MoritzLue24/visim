
pub type Key = sdl2::keyboard::Keycode;
pub type Mod = sdl2::keyboard::Mod;
pub type MouseButton = sdl2::mouse::MouseButton;
pub type MouseWheelDir = sdl2::mouse::MouseWheelDirection;


#[derive(Debug, Clone)]
pub enum Event {
    Quit,
    KeyDown { key: Key, modifier: Mod },
    KeyUp { key: Key, modifier: Mod },
    MouseMotion { x: i32, y: i32, x_rel: i32, y_rel: i32 },
    MouseButtonDown { button: MouseButton, x: i32, y: i32 },
    MouseButtonUp { button: MouseButton, x: i32, y: i32 },
    MouseWheel { direction: MouseWheelDir, x: i32, y: i32 },
    DropFile { filename: String },
}


impl Event {
    pub fn from(value: sdl2::event::Event) -> Result<Self, sdl2::event::Event> {
        match value {
            sdl2::event::Event::Quit { .. } => Ok(Self::Quit),
            sdl2::event::Event::KeyDown { keycode, keymod, .. } => Ok(Self::KeyDown { key: keycode.unwrap(), modifier: keymod }),
            sdl2::event::Event::KeyUp { keycode, keymod, .. } => Ok(Self::KeyUp { key: keycode.unwrap(), modifier: keymod }),
            sdl2::event::Event::MouseMotion { x, y, xrel, yrel, .. } => Ok(Self::MouseMotion { x, y, x_rel: xrel, y_rel: yrel }),
            sdl2::event::Event::MouseButtonDown { mouse_btn, x, y, .. } => Ok(Self::MouseButtonDown { button: mouse_btn, x, y }),
            sdl2::event::Event::MouseButtonUp { mouse_btn, x, y, .. } => Ok(Self::MouseButtonUp { button: mouse_btn, x, y }),
            sdl2::event::Event::MouseWheel { x, y, direction, .. } => Ok(Self::MouseWheel { direction, x, y }),
            sdl2::event::Event::DropFile { filename, .. } => Ok(Self::DropFile { filename }),
            _ => Err(value)
        }
    }
}
