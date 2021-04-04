use crate::texture::Texture;
use crate::animation::Animation;
use crate::types::Rect;
use crate::text::{self, DrawTextExt};
use std::path::Path;
use std::rc::Rc;

pub struct Resources{
    pub animation: Vec<Rc<Animation>>,
    pub textures: Vec<Rc<Texture>>,
    // pub text_data:text::TextData,
    //text
}
impl Resources {
    pub fn new() -> Self {
        Self{
            animation:vec![Rc::new(Animation::freeze(Rect{x:0,y:0,w:16,h:32}))],       
            textures:vec![Rc::new(Texture::with_file(Path::new("content/player.png")))]
        }
    }
    pub fn load_texture(&self, p: impl AsRef<Path>) -> Rc<Texture> {
        Rc::new(Texture::with_file(p.as_ref()))
    }
}

pub fn square(x:i32) -> i32{
    return x * x;
}