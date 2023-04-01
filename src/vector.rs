
#[derive(Clone, Copy, Debug)]
#[repr(C, packed)]
pub struct Vector2 {
    x: f32,
    y: f32
}

impl Vector2 {
    pub fn new(x: f32, y: f32) -> Self {
        Self { x, y }
    }
}

impl From<(f32, f32)> for Vector2 {
    fn from(value: (f32, f32)) -> Self {
        Self { x: value.0, y: value.1 }
    }
}
