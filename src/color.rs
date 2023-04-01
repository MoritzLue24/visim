
#[derive(Clone, Copy, Debug)]
#[repr(C, packed)]
pub struct Color {
    r: f32,
    g: f32,
    b: f32,
    a: f32
}

impl Color {
    pub fn rgba(r: f32, g: f32, b: f32, a: f32) -> Color {
        Self { r, g, b, a }
    }

    pub fn rgb(r: f32, g: f32, b: f32) -> Color {
        Self { r, g, b, a: 1.0 }
    }
}

impl From<(f32, f32, f32)> for Color {
    fn from(value: (f32, f32, f32)) -> Self {
        Self { r: value.0, g: value.1, b: value.2, a: 1.0 }
    }
}

impl From<(f32, f32, f32, f32)> for Color  {
    fn from(value: (f32, f32, f32, f32)) -> Self {
        Self { r: value.0, g: value.1, b: value.2, a: value.3 }
    }
}
