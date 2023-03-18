
#[derive(Clone, Copy, Debug)]
#[repr(C, packed)]
pub struct Vertex {
    pub pos: (f32, f32, f32),
    pub color: (f32, f32, f32, f32)
}

