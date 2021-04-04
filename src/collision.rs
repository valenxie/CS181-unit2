// pub struct Contact(ContactID, ContactID);
use crate::{resources::*, tiles::Tile};
use crate::types::*;
use std::rc::Rc; 
use crate::animation::*;
use crate::tiles::*;



// impl Contact {
//     pub fn get_ids(&self) -> (ContactID, ContactID) {
//         (self.0, self.1)
//     }
// }

#[derive(Copy, Clone)]
pub struct TileContact{
    pub tile: Tile,
    pub rect: Rect  
}
// pub enum ContactID {
//     Barrier,
//     Player,
// }

#[derive(PartialEq, Eq, Clone, Copy, Debug)]
pub struct Contact<A,B>{
    pub a: A, 
    pub b: B,
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

pub fn gather_contacts(positions: &Vec<Vec2i>, sizes: &Vec<(usize,usize)>) -> Vec<Contact<usize,usize>> {
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
// Loop through tiles that might be touching
pub fn gather_contacts_tilemap(positions: &Vec<Vec2i>, sizes: &Vec<(usize,usize)>,tilemaps:&Vec<Tilemap>)-> Vec<Contact<usize,TileContact>> {
    let mut into = vec![];
    for (i, (pos, sz)) in positions.iter().zip(sizes.iter()).enumerate() {
        let mut rect1 = Rect {
            x: positions[i].0,
            y: positions[i].1,
            w: sizes[i].0 as u16,
            h: sizes[i].1 as u16,
        };
        for tm in tilemaps.iter(){
            for (tile,rect) in [Vec2i(positions[i].0,positions[i].1), //x,y
                                          Vec2i(positions[i].0+sizes[i].0 as i32,positions[i].1), //x+w,y
                                          Vec2i(positions[i].0,positions[i].1+sizes[i].1 as i32), //x,y+h
                                          Vec2i(positions[i].0+sizes[i].0 as i32,positions[i].1+sizes[i].1 as i32)].iter().filter_map(|pos| tm.tile_at(*pos)){
                if tile.solid{
                    if let Some(disp)=rect_displacement(rect1, rect){
                        into.push(Contact{a:i,b:TileContact { tile: tile, rect: rect },mtv:disp})
                    }
                }
            }     

        }
    }
    return into;
}
pub fn restitute(positions: &mut Vec<Vec2i>, sizes: &Vec<(usize,usize)>, contacts: &mut Vec<Contact<usize,TileContact>>) {
    // handle restitution of dynamics against dynamics and dynamics against statics wrt contacts.
    // You could instead make contacts `Vec<Contact>` if you think you might remove contacts.
    // You could also add an additional parameter, a slice or vec representing how far we've displaced each dynamic, to avoid allocations if you track a vec of how far things have been moved.
    // You might also want to pass in another &mut Vec<Contact> to be filled in with "real" touches that actually happened.
    contacts.sort_unstable_by_key(|c| -(c.mtv.0 * c.mtv.0 + c.mtv.1 * c.mtv.1));
    // Keep going!  Note that you can assume every contact has a dynamic object in .a.
    // You might decide to tweak the interface of this function to separately take dynamic-static and dynamic-dynamic contacts, to avoid a branch inside of the response calculation.
    // Or, you might decide to calculate signed mtvs taking direction into account instead of the unsigned displacements from rect_displacement up above.  Or calculate one MTV per involved entity, then apply displacements to both objects during restitution (sorting by the max or the sum of their magnitudes)
    for c in contacts.iter(){
        if let Some((x,y)) = rect_displacement(a_rect, c.b.rect){
            if x > y {
                if positions[c.a].1 < c.b.rect.y{
                    c.b.rect.y += y
                } else {
                    c.b.rect.y -= y
            }
        }
        else {
            if positions[c.a].0 < c.b.rect.x  {
                c.b.rect.x += x
                } else {
                    c.b.rect.x -= x
                    
                }
            }
        }
    }
}
