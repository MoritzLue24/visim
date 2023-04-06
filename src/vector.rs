
#[derive(Clone, Copy, Debug)]
#[repr(C, packed)]
pub struct Vector2 {
    pub x: f32,
    pub y: f32
}

impl Vector2 {
    pub fn new(x: f32, y: f32) -> Self {
        Self { x, y }
    }

    pub fn mag(&self) -> f32 {
        let scalar = self.x * self.x + self.y * self.y;
        scalar.sqrt()
    }

    pub fn norm(&mut self) {
        let mag = self.mag();
        if mag != 0.0 {
            // self /= mag;
        }
    }
}

impl From<(f32, f32)> for Vector2 {
    fn from(value: (f32, f32)) -> Self {
        Self { x: value.0, y: value.1 }
    }
}

impl std::ops::Add<Self> for Vector2 {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self { x: self.x + rhs.x, y: self.y + rhs.y }
    }
}

impl std::ops::Sub<Self> for Vector2 {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Self { x: self.x - rhs.x, y: self.y - rhs.y }
    }
}

impl std::ops::Mul<Self> for Vector2 {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        Self { x: self.x * rhs.x, y: self.y * rhs.y }
    }
}

impl std::ops::Div<Self> for Vector2 {
    type Output = Self;

    fn div(self, rhs: Self) -> Self::Output {
        Self { x: self.x / rhs.x, y: self.y / rhs.y }
    }
}

impl std::ops::AddAssign<Self> for Vector2 {
    fn add_assign(&mut self, rhs: Self) {
        self.x += rhs.x;
        self.y += rhs.y;
    }
}

impl std::ops::SubAssign<Self> for Vector2 {
    fn sub_assign(&mut self, rhs: Self) {
        self.x -= rhs.x;
        self.y -= rhs.y;
    }
}

impl std::ops::MulAssign<Self> for Vector2 {
    fn mul_assign(&mut self, rhs: Self) {
        self.x *= rhs.x;
        self.y *= rhs.y;
    }
}

impl std::ops::DivAssign<Self> for Vector2 {
    fn div_assign(&mut self, rhs: Self) {
        self.x /= rhs.x;
        self.y /= rhs.y;
    }
}

impl std::ops::Add<f32> for Vector2 {
    type Output = Self;

    fn add(self, rhs: f32) -> Self::Output {
        Self { x: self.x + rhs, y: self.y + rhs }
    }
}

impl std::ops::Sub<f32> for Vector2 {
    type Output = Self;

    fn sub(self, rhs: f32) -> Self::Output {
        Self { x: self.x - rhs, y: self.y - rhs }
    }
}

impl std::ops::Mul<f32> for Vector2 {
    type Output = Self;

    fn mul(self, rhs: f32) -> Self::Output {
        Self { x: self.x * rhs, y: self.y * rhs }
    }
}

impl std::ops::Div<f32> for Vector2 {
    type Output = Self;

    fn div(self, rhs: f32) -> Self::Output {
        Self { x: self.x / rhs, y: self.y / rhs }
    }
}

impl std::ops::AddAssign<f32> for Vector2 {
    fn add_assign(&mut self, rhs: f32) {
        self.x += rhs;
        self.y += rhs;
    }
}

impl std::ops::SubAssign<f32> for Vector2 {
    fn sub_assign(&mut self, rhs: f32) {
        self.x -= rhs;
        self.y -= rhs;
    }
}

impl std::ops::MulAssign<f32> for Vector2 {
    fn mul_assign(&mut self, rhs: f32) {
        self.x *= rhs;
        self.y *= rhs;
    }
}

impl std::ops::DivAssign<f32> for Vector2 {
    fn div_assign(&mut self, rhs: f32) {
        self.x /= rhs;
        self.y /= rhs;
    }
}

impl std::ops::Neg for Vector2 {
    type Output = Self;

    fn neg(self) -> Self::Output {
        Self { x: -self.x, y: -self.y }
    }
}
