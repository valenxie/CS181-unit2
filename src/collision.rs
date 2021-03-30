// pub struct Contact(ContactID, ContactID);
use crate::resources::*;
use crate::types::*;
use std::rc::Rc; 
use crate::animation::*;

// impl Contact {
//     pub fn get_ids(&self) -> (ContactID, ContactID) {
//         (self.0, self.1)
//     }
// }

// #[derive(Copy, Clone)]
// pub enum ContactID {
//     Barrier,
//     Player,
// }

#[derive(PartialEq, Eq, Clone, Copy, Debug)]
pub struct Contact {
    pub a: usize, 
    pub b: usize,
    pub mtv:(i32,i32)
}

// pub fn gather_contacts(player:, Barrier:) -> Vec<Contact> {}
    
pub fn rect_touching(r1:Rect, r2:Rect) -> bool {
    // r1 left is left of r2 right
    return r1.x <= r2.x+r2.w as i32 &&
    // r2 left is left of r1 right
    r2.x <= r1.x+r1.w as i32 &&
    // those two conditions handle the x axis overlap;
    // the next two do the same for the y axis:
    r1.y <= r2.y+r2.h as i32 &&
    r2.y <= r1.y+r1.h as i32
}

pub fn resources_touching(r1:Rc<Animation>, r2:Rc<Animation>) -> bool {
    return rect_touching(r1.frames[0].0, r2.frames[0].0);
}

pub fn rect_displacement(r1:Rect, r2:Rect) -> Option<(i32,i32)> {
    let x_overlap = (r1.x+r1.w as i32).min(r2.x+r2.w as i32) - r1.x.max(r2.x);
    let y_overlap = (r1.y+r1.h as i32).min(r2.y+r2.h as i32) - r1.y.max(r2.y);
    if x_overlap >= 0 && y_overlap >= 0 {
        // This will return the magnitude of overlap in each axis.
        Some((x_overlap, y_overlap))
    } else {
        None
    }
}

pub fn resources_displacement(r1:Rc<Animation>, r2:Rc<Animation>) -> Option<(i32,i32)> {
    return rect_displacement(r1.frames[0].0, r2.frames[0].0);
}

pub fn gather_contacts(positions: &Vec<Vec2i>, sizes: &Vec<(usize,usize)>) -> Vec<Contact> {
    let mut into = vec![];
    for i in 0..positions.len() {
        let mut rect1 = Rect {
            x: positions[i].0,
            y: positions[i].1,
            w: sizes[i].0 as u16,
            h: sizes[i].1 as u16,
        };
        for j in 0..positions.len() {
            if j==i {
                continue;
            } else {
                let mut rect2 = Rect {
                    x: positions[j].0,
                    y: positions[j].1,
                    w: sizes[j].0 as u16,
                    h: sizes[j].1 as u16,
                };
                if let Some(disp) = rect_displacement(rect1, rect2) {
                    into.push(Contact{a:i, b:j, mtv:disp});
                }
            }
        }
    }
    return into;
}