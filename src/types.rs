#[derive(PartialEq, Eq, Clone, Copy, Hash, Debug)]
pub struct Rect {
    pub x: i32,
    pub y: i32,
    pub w: u16,
    pub h: u16,
}
impl Rect {
    pub fn new(x: i32, y: i32, w: u16, h: u16) -> Self {
        Self { x, y, w, h }
    }
}
#[derive(PartialEq, Eq, Clone, Copy, Hash, Debug)]
pub struct Vec2i(pub i32, pub i32);

#[derive(PartialEq, Clone, Copy, Debug)]
pub struct Vec2f(pub f32, pub f32);
#[derive(PartialEq, Eq, Clone, Copy, Hash, Debug)]
pub struct Rgba(pub u8, pub u8, pub u8, pub u8);

// Feel free to add impl blocks with convenience functions
