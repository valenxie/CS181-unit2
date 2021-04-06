use crate::texture::Texture;
use crate::animation::Animation;
use crate::types::Rect;
use crate::text::{self, DrawText};
use std::path::Path;
use std::rc::Rc;
const CHAR_SIZE: u16 = 8;
pub struct Resources{
    pub animation: Vec<Rc<Animation>>,
    pub textures: Vec<Rc<Texture>>,
    pub text:text::Text,
    //text
}
impl Resources {
    pub fn new() -> Self {
        Self{
            animation:vec![Rc::new(Animation::freeze(Rect{x:0,y:0,w:16,h:32}))],       
            textures:vec![Rc::new(Texture::with_file(Path::new("content/player.png")))],
            text: {
                let image =
                    Rc::new(Texture::with_file(Path::new("content/font.png")));
                let info = [
                    (' ', Rect::new(56, 104, CHAR_SIZE, CHAR_SIZE)),
                    ('a', Rect::new(40, 24, CHAR_SIZE, CHAR_SIZE)),
                    ('b', Rect::new(48, 24, CHAR_SIZE, CHAR_SIZE)),
                    ('c', Rect::new(56, 24, CHAR_SIZE, CHAR_SIZE)),
                    ('d', Rect::new(64, 24, CHAR_SIZE, CHAR_SIZE)),
                    ('e', Rect::new(72, 24, CHAR_SIZE, CHAR_SIZE)),
                    ('f', Rect::new(80, 24, CHAR_SIZE, CHAR_SIZE)),
                    ('g', Rect::new(88, 24, CHAR_SIZE, CHAR_SIZE)),
                    ('h', Rect::new(0, 32, CHAR_SIZE, CHAR_SIZE)),
                    ('i', Rect::new(8, 32, CHAR_SIZE, CHAR_SIZE)),
                    ('j', Rect::new(16, 32, CHAR_SIZE, CHAR_SIZE)),
                    ('k', Rect::new(24, 32, CHAR_SIZE, CHAR_SIZE)),
                    ('l', Rect::new(32, 32, CHAR_SIZE, CHAR_SIZE)),
                    ('m', Rect::new(40, 32, CHAR_SIZE, CHAR_SIZE)),
                    ('n', Rect::new(48, 32, CHAR_SIZE, CHAR_SIZE)),
                    ('o', Rect::new(56, 32, CHAR_SIZE, CHAR_SIZE)),
                    ('p', Rect::new(64, 32, CHAR_SIZE, CHAR_SIZE)),
                    ('q', Rect::new(72, 32, CHAR_SIZE, CHAR_SIZE)),
                    ('r', Rect::new(80, 32, CHAR_SIZE, CHAR_SIZE)),
                    ('s', Rect::new(88, 32, CHAR_SIZE, CHAR_SIZE)),
                    ('t', Rect::new(0, 40, CHAR_SIZE, CHAR_SIZE)),
                    ('u', Rect::new(8, 40, CHAR_SIZE, CHAR_SIZE)),
                    ('v', Rect::new(16, 40, CHAR_SIZE, CHAR_SIZE)),
                    ('w', Rect::new(24, 40, CHAR_SIZE, CHAR_SIZE)),
                    ('x', Rect::new(32, 40, CHAR_SIZE, CHAR_SIZE)),
                    ('y', Rect::new(40, 40, CHAR_SIZE, CHAR_SIZE)),
                    ('z', Rect::new(48, 40, CHAR_SIZE, CHAR_SIZE)),
                ];
                text::Text::new(&image, &info)
            },
        }
    }
    pub fn load_texture(&self, p: impl AsRef<Path>) -> Rc<Texture> {
        Rc::new(Texture::with_file(p.as_ref()))
    }
}

pub fn square(x:i32) -> i32{
    return x * x;
}