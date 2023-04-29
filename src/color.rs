fn clamp(n: f32) -> f32 {
    if n < 0.0 {
        return 0.0
    }
    if n > 1.0 {
        return 1.0
    }
    n
}

fn clamp_col(c: &mut Color) {
    c.r = clamp(c.r);
    c.g = clamp(c.g);
    c.b = clamp(c.b);
    c.a = clamp(c.a);
}


#[derive(Clone, Copy, Debug)]
#[repr(C, packed)]
pub struct Color {
    pub r: f32,
    pub g: f32,
    pub b: f32,
    pub a: f32
}

impl Color {
    pub fn rgba(r: f32, g: f32, b: f32, a: f32) -> Color {
        Self { r: clamp(r), g: clamp(g), b: clamp(b), a: clamp(a) }
    }

    pub fn rgb(r: f32, g: f32, b: f32) -> Color {
        Self { r: clamp(r), g: clamp(g), b: clamp(b), a: 1.0 }
    }
}

impl From<(f32, f32, f32)> for Color {
    fn from(value: (f32, f32, f32)) -> Self {
        Self { r: clamp(value.0), g: clamp(value.1), b: clamp(value.2), a: 1.0 }
    }
}

impl From<(f32, f32, f32, f32)> for Color  {
    fn from(value: (f32, f32, f32, f32)) -> Self {
        Self { r: clamp(value.0), g: clamp(value.1), b: clamp(value.2), a: clamp(value.3) }
    }
}

impl std::ops::Add<Self> for Color {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self { r: clamp(self.r + rhs.r), g: clamp(self.g + rhs.g), b: clamp(self.b + rhs.b), a: clamp(self.a + rhs.a) }
    }
}

impl std::ops::Sub<Self> for Color {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Self { r: clamp(self.r - rhs.r), g: clamp(self.g - rhs.g), b: clamp(self.b - rhs.b), a: clamp(self.a - rhs.a) }
    }
}

impl std::ops::Mul<Self> for Color {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        Self { r: clamp(self.r * rhs.r), g: clamp(self.g * rhs.g), b: clamp(self.b * rhs.b), a: clamp(self.a * rhs.a) }
    }
}

impl std::ops::Div<Self> for Color {
    type Output = Self;

    fn div(self, rhs: Self) -> Self::Output {
        Self { r: clamp(self.r / rhs.r), g: clamp(self.g / rhs.g), b: clamp(self.b / rhs.b), a: clamp(self.a / rhs.a) }
    }
}

impl std::ops::AddAssign<Self> for Color {
    fn add_assign(&mut self, rhs: Self) {
        self.r += rhs.r;
        self.g += rhs.g;
        self.b += rhs.b;
        self.a += rhs.a;
        clamp_col(self);
    }
}

impl std::ops::SubAssign<Self> for Color {
    fn sub_assign(&mut self, rhs: Self) {
        self.r -= rhs.r;
        self.g -= rhs.g;
        self.b -= rhs.b;
        self.a -= rhs.a;
        clamp_col(self);
    }
}

impl std::ops::MulAssign<Self> for Color {
    fn mul_assign(&mut self, rhs: Self) {
        self.r *= rhs.r;
        self.g *= rhs.g;
        self.b *= rhs.b;
        self.a *= rhs.a;
        clamp_col(self);
    }
}

impl std::ops::DivAssign<Self> for Color {
    fn div_assign(&mut self, rhs: Self) {
        self.r /= rhs.r;
        self.g /= rhs.g;
        self.b /= rhs.b;
        self.a /= rhs.a;
        clamp_col(self);
    }
}

impl std::ops::Add<f32> for Color {
    type Output = Self;

    fn add(self, rhs: f32) -> Self::Output {
        Self { r: clamp(self.r + rhs), g: clamp(self.g + rhs), b: clamp(self.b + rhs), a: clamp(self.a + rhs) }
    }
}

impl std::ops::Sub<f32> for Color {
    type Output = Self;

    fn sub(self, rhs: f32) -> Self::Output {
        Self { r: clamp(self.r - rhs), g: clamp(self.g - rhs), b: clamp(self.b - rhs), a: clamp(self.a - rhs) }
    }
}

impl std::ops::Mul<f32> for Color {
    type Output = Self;

    fn mul(self, rhs: f32) -> Self::Output {
        Self { r: clamp(self.r * rhs), g: clamp(self.g * rhs), b: clamp(self.b * rhs), a: clamp(self.a * rhs) }
    }
}

impl std::ops::Div<f32> for Color {
    type Output = Self;

    fn div(self, rhs: f32) -> Self::Output {
        Self { r: clamp(self.r / rhs), g: clamp(self.g / rhs), b: clamp(self.b / rhs), a: clamp(self.a / rhs) }
    }
}

impl std::ops::AddAssign<f32> for Color {
    fn add_assign(&mut self, rhs: f32) {
        self.r += rhs;
        self.g += rhs;
        self.b += rhs;
        self.a += rhs;
        clamp_col(self);
    }
}

impl std::ops::SubAssign<f32> for Color {
    fn sub_assign(&mut self, rhs: f32) {
        self.r -= rhs;
        self.g -= rhs;
        self.b -= rhs;
        self.a -= rhs;
        clamp_col(self);
    }
}

impl std::ops::MulAssign<f32> for Color {
    fn mul_assign(&mut self, rhs: f32) {
        self.r *= rhs;
        self.g *= rhs;
        self.b *= rhs;
        self.a *= rhs;
        clamp_col(self);
    }
}

impl std::ops::DivAssign<f32> for Color {
    fn div_assign(&mut self, rhs: f32) {
        self.r /= rhs;
        self.g /= rhs;
        self.b /= rhs;
        self.a /= rhs;
        clamp_col(self);
    }
}
