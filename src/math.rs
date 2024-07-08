#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum KernelKind {
    Gaussian,
}

pub type Rectf = Rect<f32>;
pub type Vec2f = Vec2<f32>;
pub type Vec3f = Vec3<f32>;
pub type Mat2f = Mat<f32, 2, 2>;
pub type Mat3f = Mat<f32, 3, 3>;
pub type Mat3x4f = Mat<f32, 3, 4>;
pub type Mat4x3f = Mat<f32, 4, 3>;

#[derive(Debug, Clone, Copy)]
pub struct Rect<F> {
    pub left: F,
    pub right: F,
    pub top: F,
    pub bottom: F,
}

impl<F> Rect<F> {
    pub fn new(left: F, right: F, top: F, bottom: F) -> Self {
        Self {
            left,
            right,
            top,
            bottom,
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Vec2<F> {
    pub x: F,
    pub y: F,
}

#[derive(Debug, Clone, Copy)]
pub struct Vec3<F> {
    pub x: F,
    pub y: F,
    pub z: F,
}

#[derive(Debug, Clone, Copy)]
pub struct Mat<F, const W: usize, const H: usize> {
    data: [[F; W]; H],
}
