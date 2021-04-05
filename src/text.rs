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

    fn get_char_width(&self, c: char) -> i32 {
        self.data.get(&c).map_or(0, |rect| rect.w.into())
    }

    fn get_string_width(&self, string: &str) -> i32 {
        let mut width = 0;
        for c in string.chars() {
            width += self.get_char_width(c);
        }
        width
    }
}

pub trait DrawText{
    fn draw_text(&mut self, string: &str, pos: Vec2i, font: &Text);
}

use crate::graphics::Screen;
impl<'fb> DrawText for Screen<'fb> {
    // makes a bunch of assumptions, such as that all the characters are the same height. works because we're using a monospace/height font, won't necessarily work for others
    fn draw_text(&mut self, string: &str, pos: Vec2i, font: &Text) {
        // starting positions
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