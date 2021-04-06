use std::collections::BTreeMap;
use std::rc::Rc;

use crate::types::{Rect, Vec2i};
use crate::texture::Texture;

pub struct Text{
    pub data: BTreeMap<char, Rect>,
    image: Rc<Texture>,
}

impl Text{
    pub fn new(image: &Rc<Texture>, char_data: &[(char, Rect)]) -> Self {
        let mut text_data = Text {
            data: BTreeMap::new(),
            image: Rc::clone(image),
        };
        for (char, rect) in char_data.iter() {
            text_data.data.insert(*char, *rect);
        }
        text_data
    }

}

pub trait DrawText{
    fn draw_text(&mut self, string: &str, pos: Vec2i, font: &Text);
}

use crate::graphics::Screen;
impl<'fb> DrawText for Screen<'fb> {  
    fn draw_text(&mut self, string: &str, pos: Vec2i, font: &Text) {
    
        let mut x = pos.0;
        let y = pos.1;
        for c in string.chars() {
            if let Some(rect) = font.data.get(&c) {
                self.bitblt(&font.image, *rect, Vec2i(x, y));
                x += rect.w as i32;
            }
        }
    }
}