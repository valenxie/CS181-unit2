use core::time;
use std::path::Path;
use std::rc::Rc;

use winit::window::WindowBuilder;
use winit::dpi::LogicalSize;
use winit::event::VirtualKeyCode;
use winit_input_helper::WinitInputHelper;

use engine2d::{animation, texture, types::*};
use engine2d::graphics::Screen;
use engine2d::tiles::*;
use engine2d::animation::*;

// use engine2d::collision::*;
// Imagine a Resources struct (we'll call it AssetDB or Assets in the future)
// which wraps all accesses to textures, sounds, animations, etc.
use engine2d::resources::*;
use engine2d::texture::Texture;

const WIDTH: usize = 256;
const HEIGHT: usize = 256;

#[derive(Clone,Copy,PartialEq,Eq,Debug)]
enum EntityType {
    Player,
    Enemy,
    Barrier,
    lvl1Exit,

}

type Level = (Vec<Tilemap>, Vec<(EntityType, i32, i32)>);
type Input = WinitInputHelper;

#[derive(Debug,Clone,Copy)]
enum Mode {
    Title,
    Lvl1,
    Lvl2,
    EndGame
}
#[derive(Debug,Clone,Copy)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Mode {
    // update consumes self and yields a new state (which might also just be self)
    fn update(self, game:&mut GameState, input:&Input,levels: &Vec<Level>) -> Self {
        match self {
            Mode::Title => {
                if input.key_held(VirtualKeyCode::P){
                    game.level=1;
                    Mode::Lvl1
                } else {
                    self
                }
            },
            Mode::Lvl1 => {
                if game.movable {                    
                    if input.key_held(VirtualKeyCode::Right) {
                        if game.positions[3].0 < 500 {
                            game.velocities[3].0 = 1;
                        } else {
                            game.velocities[3].0 = 0;
                        }
                        if game.positions[3].0 >= game.camera.0+16*5 && 
                            game.camera.0 < 256 {
                            game.camera.0 += 1;
                        }
                    } else if input.key_held(VirtualKeyCode::Left) {
                        if game.positions[3].0 > -256 {
                            game.velocities[3].0 = -1;
                        } else {
                            game.velocities[3].0 = 0;
                        }
                        if game.positions[3].0 <= game.camera.0+16*5 && 
                            game.camera.0 > -256 {
                            game.camera.0 -= 1;
                        }
                    } else {
                        game.velocities[3].0 = 0;
                    }
                    if input.key_held(VirtualKeyCode::Up) {
                        if game.positions[3].1 > -256 {
                            game.velocities[3].1 = -1;
                        } else {
                            game.velocities[3].1 = 0;
                        }
                        if game.positions[3].1 <= game.camera.1+16*5 && 
                            game.camera.1 > -256 {
                            game.camera.1 -= 1;
                        }
                    } else if input.key_held(VirtualKeyCode::Down) {
                        if game.positions[3].1 < 208 {
                            game.velocities[3].1 = 1;
                        } else {
                            game.velocities[3].1 = 0;
                        }
                    
                        if game.positions[3].1 >= game.camera.1+16*5 && 
                            game.camera.1 < 0 {
                            game.camera.1 += 1;
                        }
                    } else {
                        game.velocities[3].1 = 0;
                    }
                } else { // on not movable
                    game.velocities[3] = Vec2i(0,0);
                }
                for (posn, vel) in game.positions.iter_mut().zip(game.velocities.iter()) {
                    posn.0 += vel.0;
                    posn.1 += vel.1;
                }
                if input.key_held(VirtualKeyCode::R){
                    game.positions[3].0 = 130;
                    game.positions[3].1 = 170;
                    game.camera = Vec2i(0, 0);
                    game.movable = true;
                }
                let contacts = engine2d::collision::gather_contacts(&game.positions, &game.sizes);
                let mut tile_contacts= engine2d::collision::gather_contacts_tilemap(&game.positions, &game.sizes,&levels[1].0);
                for contact in contacts.iter(){
                    match (levels[game.level].1[contact.a].0, levels[game.level].1[contact.b].0){
                        (EntityType::Player, EntityType::Barrier) => {
                            game.movable = false;
                            //Generate text on screen
                            let barrier_touched_anim = Rc::new(Animation {
                                frames: vec![(Rect{x:0,y:0,w:32,h:32}, 1),
                                             (Rect {x: 32,y: 0,w: 32, h: 32},1)],
                                looping: false,
                            });
                            barrier_touched_anim.start();
                            
                        }
                        (EntityType::Player, EntityType::lvl1Exit) => {
                            // game.movable = false;
                            game.level = 1;
                            game.mode= Mode::Lvl2;
                            // Mode::Lvl2;
                        }
                        _ => {}
                    }       
                }
                if input.key_held(VirtualKeyCode::Q){
                    Mode::EndGame
                } else {
                    self
                }

            },
            Mode::Lvl2 => {
                if input.key_held(VirtualKeyCode::Q){
                    Mode::EndGame
                } else {
                    self
                }
            }
            Mode::EndGame => {
                if input.key_held(VirtualKeyCode::E) {
                    panic!();
                } else {
                    self
                }
            }
        }
    }
    fn display(&self, game:&GameState, screen: &mut Screen, levels: &Vec<Level>) {
        match self {
            Mode::Title => {
                for t in levels[0].0.iter(){
                    t.draw(screen);
                }
                //TODO: draw text
            },
            Mode::Lvl1=> {
                // screen.set_scroll(game.camera);
                for t in levels[1].0.iter(){
                    t.draw(screen);
                }
                for ((pos,tex),anim) in game.positions.iter().zip(game.textures.iter()).zip(game.anim_state.iter()) {
                    screen.bitblt(tex,anim.frame(),*pos);
                }
            },
            Mode::Lvl2=> {
                // screen.set_scroll(game.camera);
                for t in levels[2].0.iter(){
                    t.draw(screen);
                }
                for ((pos,tex),anim) in game.positions.iter().zip(game.textures.iter()).zip(game.anim_state.iter()) {
                    screen.bitblt(tex,anim.frame(),*pos);
                }
            },
            Mode::EndGame => {
                for t in levels[2].0.iter(){
                    t.draw(screen);
                }
            }
        }
    }
}


struct GameState{
        // Every entity has a position, a size, a texture, and animation state.
        // Assume entity 0 is the player
        types: Vec<EntityType>,
        positions: Vec<Vec2i>,
        velocities: Vec<Vec2i>,
        sizes:Vec<(usize,usize)>,
        textures:Vec<Rc<Texture>>,
        anim_state:Vec<AnimationState>,
        // Current level
        level:usize,
        // Camera position
        camera:Vec2i,
        mode:Mode,
        movable:bool,
        direction: Direction
    }

fn main() {
    let window_builder = {
        let size = LogicalSize::new(WIDTH as f64, HEIGHT as f64);
        WindowBuilder::new()
            .with_title("MazeChill")
            .with_inner_size(size)
            .with_min_inner_size(size)
            .with_resizable(false)
    };
    // Here's our resources...
    let rsrc = Resources::new();
    let tileset1 = Rc::new(Tileset::new(
        vec![
            Tile{solid:false},
            Tile{solid:false},
            Tile{solid:false},
            Tile{solid:false},
        ],
        &rsrc.load_texture(Path::new("content/water.png"))
    ));
    let lvl1tileset = Rc::new(Tileset::new(
        vec![
            Tile{solid:false},
            Tile{solid:true},
            Tile{solid:false},
            Tile{solid:false},
            Tile{solid:false},
            Tile{solid:true},
            Tile{solid:false},
            Tile{solid:true},
            Tile{solid:false},
            Tile{solid:false},
            Tile{solid:true},
            Tile{solid:true},
            Tile{solid:true},
            Tile{solid:false},
            Tile{solid:false},
            Tile{solid:false},
            Tile{solid:false},
            Tile{solid:false},
            Tile{solid:false},
            Tile{solid:false},
            Tile{solid:false},
            Tile{solid:false},
            Tile{solid:false},
            Tile{solid:false},
            Tile{solid:false},
            Tile{solid:false},
            Tile{solid:false},
            Tile{solid:false},
            Tile{solid:false},
            Tile{solid:false},
            Tile{solid:false},
            Tile{solid:false},
            Tile{solid:false},
            Tile{solid:false},
            Tile{solid:false},
            Tile{solid:false},
            Tile{solid:false},
            Tile{solid:false},
            Tile{solid:false},
            Tile{solid:false},
            Tile{solid:false},
            Tile{solid:false},
            Tile{solid:false},
            Tile{solid:false},
            Tile{solid:false},
            Tile{solid:false},
            Tile{solid:false},
            Tile{solid:false},
            Tile{solid:false},
            Tile{solid:false},
            Tile{solid:false},
            Tile{solid:false},
            Tile{solid:false},
            Tile{solid:false},
            Tile{solid:false},
            Tile{solid:false},
            Tile{solid:false},
            Tile{solid:false},
            Tile{solid:false},
            Tile{solid:false},
            Tile{solid:false},
            Tile{solid:false},
            Tile{solid:false},
            Tile{solid:false},
            Tile{solid:false},
            Tile{solid:false},
            Tile{solid:false},
            Tile{solid:false},
            Tile{solid:false},
            Tile{solid:false},
            Tile{solid:false},
            Tile{solid:false},
            Tile{solid:false},
            Tile{solid:false},
            Tile{solid:false},
            Tile{solid:false},
            Tile{solid:false},
            Tile{solid:false},
            Tile{solid:false},
            Tile{solid:false},
            Tile{solid:false},
            Tile{solid:false},
            Tile{solid:false},
            Tile{solid:false},
            Tile{solid:false},
            Tile{solid:false},
            Tile{solid:false},
            Tile{solid:false},
            Tile{solid:false},
            Tile{solid:false},
            Tile{solid:false},
            Tile{solid:false},
            Tile{solid:false},
            Tile{solid:false},
            Tile{solid:false},
            Tile{solid:false},
            Tile{solid:false},
            Tile{solid:false},
            Tile{solid:false},
            Tile{solid:false},
            Tile{solid:false},
            Tile{solid:false},
            Tile{solid:false},
            Tile{solid:false},
            Tile{solid:false},
            Tile{solid:false},
            Tile{solid:false},
            Tile{solid:false},
            Tile{solid:false},
            Tile{solid:false},
            Tile{solid:false},
            Tile{solid:false},
            Tile{solid:false},
            Tile{solid:false},
            Tile{solid:false},
            Tile{solid:false},
            Tile{solid:false},
            Tile{solid:false},
            Tile{solid:false},
            Tile{solid:false},
            Tile{solid:false},
            Tile{solid:false},
            Tile{solid:false},
            Tile{solid:false},
            Tile{solid:false},
            Tile{solid:false},
            Tile{solid:false},
            Tile{solid:false},
            Tile{solid:false},
            Tile{solid:false},
            Tile{solid:false},
        ],
        &rsrc.load_texture(Path::new("content/lvl1.png"))
    ));
    let lvl2tileset = Rc::new(Tileset::new(
        vec![
            Tile{solid:false},
            Tile{solid:false},
            Tile{solid:false},
            Tile{solid:false},
            Tile{solid:false},
            Tile{solid:false},
            Tile{solid:false},
            Tile{solid:false},
            Tile{solid:false},
            Tile{solid:false},
            Tile{solid:false},
            Tile{solid:false},
            Tile{solid:false},
            Tile{solid:false},
            Tile{solid:false},
            Tile{solid:false},
            Tile{solid:false},
            Tile{solid:false},
            Tile{solid:false},
            Tile{solid:false},
            Tile{solid:false},
            Tile{solid:false},
            Tile{solid:false},
            Tile{solid:false},
            Tile{solid:false},
            Tile{solid:false},
            Tile{solid:false},
            Tile{solid:false},
            Tile{solid:false},
            Tile{solid:false},
            Tile{solid:false},
            Tile{solid:false},
            Tile{solid:false},
            Tile{solid:false},
            Tile{solid:false},
            Tile{solid:false},
            Tile{solid:false},
            Tile{solid:false},
            Tile{solid:false},
            Tile{solid:false},
            Tile{solid:false},
            Tile{solid:false},
            Tile{solid:false},
            Tile{solid:false},
            Tile{solid:false},
            Tile{solid:false},
            Tile{solid:false},
            Tile{solid:false},
            Tile{solid:false},
            Tile{solid:false},
            Tile{solid:false},
            Tile{solid:false},
            Tile{solid:false},
            Tile{solid:false},
            Tile{solid:false},
            Tile{solid:false},
            Tile{solid:false},
            Tile{solid:false},
            Tile{solid:false},
            Tile{solid:false},
            Tile{solid:false},
            Tile{solid:false},
            Tile{solid:false},
            Tile{solid:false},
            Tile{solid:false},
            Tile{solid:false},
            Tile{solid:false},
            Tile{solid:false},
            Tile{solid:false},
        ],
        &rsrc.load_texture(Path::new("content/lvl2.png"))
    ));

    // tilemaps
    let start_screen = Tilemap::new(
        Vec2i(0, 0),
        (16, 16),
        &lvl1tileset,
        vec![
            130, 130, 130, 130, 130, 130, 130, 130, 130, 130, 130, 130, 130, 130, 130, 130,
            130, 130, 130, 130, 130, 130, 130, 130, 130, 130, 130, 130, 130, 130, 130, 130,
            130, 130, 130, 130, 130, 130, 130, 130, 130, 130, 130, 130, 130, 130, 130, 130,
            130, 130, 130, 130, 130, 130, 130, 130, 130, 130, 130, 130, 130, 130, 130, 130,
            130, 130, 130, 130, 130, 130, 130, 130, 130, 130, 130, 130, 130, 130, 130, 130,
            130, 130, 130, 130, 130, 130, 130, 130, 130, 130, 130, 130, 130, 130, 130, 130,
            130, 130, 130, 130, 130, 130, 130, 130, 130, 130, 130, 130, 130, 130, 130, 130,
            130, 130, 130, 130, 130, 130, 130, 130, 130, 130, 130, 130, 130, 130, 130, 130,
            130, 130, 130, 130, 130, 130, 130, 130, 130, 130, 130, 130, 130, 130, 130, 130,
            130, 130, 130, 130, 130, 130, 130, 130, 130, 130, 130, 130, 130, 130, 130, 130,
            130, 130, 130, 130, 130, 130, 130, 130, 130, 130, 130, 130, 130, 130, 130, 130,
            130, 130, 130, 130, 130, 130, 130, 130, 130, 130, 130, 130, 130, 130, 130, 130,
            130, 130, 130, 130, 130, 130, 130, 130, 130, 130, 130, 130, 130, 130, 130, 130,
            130, 130, 130, 130, 130, 130, 130, 130, 130, 130, 130, 130, 130, 130, 130, 130,
            130, 130, 130, 130, 130, 130, 130, 130, 130, 130, 130, 130, 130, 130, 130, 130,
            130, 130, 130, 130, 130, 130, 130, 130, 130, 130, 130, 130, 130, 130, 130, 130,
        ],
    );
    let lvl1map_1 = Tilemap::new(
        Vec2i(0, 0),
        (16, 16),
        &lvl1tileset,
        vec![
            130, 130, 130, 130, 130, 130, 5, 6, 6, 6, 7, 130, 130, 130, 130, 130,
            130, 130, 130, 130, 130, 130, 5, 6, 6, 6, 7, 130, 130, 130, 130, 130,
            130, 130, 130, 130, 130, 130, 5, 6, 6, 6, 7, 130, 130, 130, 130, 130,
            1, 1, 1, 1, 1, 1, 6, 6, 6, 6, 6, 1, 1, 1, 1, 1,
            6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6,
            6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6,
            11, 11, 11, 11, 11, 11, 6, 6, 6, 6, 6, 11, 11, 11, 11, 11,
            130, 130, 130, 130, 130, 130, 5, 6, 6, 6, 7, 130, 130, 130, 130, 130,
            130, 130, 130, 130, 130, 130, 5, 6, 6, 6, 7, 130, 130, 130, 130, 130,
            130, 130, 130, 130, 130, 130, 5, 6, 6, 6, 7, 130, 130, 130, 130, 130,
            130, 130, 130, 130, 130, 130, 5, 6, 6, 6, 7, 130, 130, 130, 130, 130,
            130, 130, 130, 130, 130, 130, 5, 6, 6, 6, 7, 130, 130, 130, 130, 130,
            130, 130, 130, 130, 130, 130, 5, 6, 6, 6, 7, 130, 130, 130, 130, 130,
            130, 130, 130, 130, 130, 130, 5, 6, 6, 6, 7, 130, 130, 130, 130, 130,
            130, 130, 130, 130, 130, 130, 5, 6, 6, 6, 7, 130, 130, 130, 130, 130,
            130, 130, 130, 130, 130, 130, 10, 11, 11, 11, 12, 130, 130, 130, 130, 130,

        ],
    );
    let lvl1map_2 = Tilemap::new(
        Vec2i(256, 0),
        (16, 16),
        &lvl1tileset,
        vec![
            130, 130, 130, 5, 6, 6, 6, 6, 1, 1, 1, 6, 6, 7, 130, 130, 
            130, 130, 130, 5, 6, 6, 6, 6, 6, 6, 6, 6, 6, 7, 130, 130, 
            130, 130, 130, 5, 6, 6, 6, 11, 11, 6, 6, 6, 6, 7, 130, 130, 
            1, 1, 1, 6, 6, 6, 7, 130, 130, 5, 6, 6, 6, 7, 130, 130, 
            6, 6, 6, 6, 6, 6, 7, 130, 130, 5, 6, 6, 6, 7, 130, 130, 
            6, 6, 6, 6, 6, 6, 7, 130, 130, 5, 6, 6, 6, 7, 130, 130, 
            11, 11, 11, 6, 6, 6, 7, 130, 130, 5, 6, 6, 6, 6, 1, 1, 
            130, 130, 130, 5, 6, 6, 7, 130, 130, 5, 6, 6, 6, 6, 6, 6,  
            130, 130, 130, 5, 6, 6, 7, 130, 130, 5, 6, 6, 6, 6, 6, 6, 
            130, 130, 130, 5, 6, 6, 7, 130, 130, 10, 11, 11, 11, 11, 11, 11,
            130, 130, 130, 5, 6, 6, 7, 130, 130, 130, 130, 130, 130, 130, 130, 130, 
            130, 130, 130, 5, 6, 6, 7, 130, 130, 130, 130, 130, 130, 130, 130, 130,
            130, 130, 130, 5, 6, 6, 7, 130, 130, 130, 130, 130, 130, 130, 130, 130,
            130, 130, 130, 5, 6, 6, 7, 130, 130, 130, 130, 130, 130, 130, 130, 130,
            130, 130, 130, 5, 6, 6, 7, 130, 130, 130, 130, 130, 130, 130, 130, 130,
            130, 130, 130, 10, 11, 11, 12, 130, 130, 130, 130, 130, 130, 130, 130, 130,

        ],
    );
    let lvl1map_3 = Tilemap::new(
        Vec2i(-256, 0),
        (16, 16),
        &lvl1tileset,
        vec![
            130, 130, 130, 130, 130, 130, 130, 130, 130, 5, 6, 6, 7, 130, 130, 130,
            130, 130, 130, 130, 130, 130, 130, 130, 130, 5, 6, 6, 7, 130, 130, 130,
            130, 130, 130, 130, 130, 130, 130, 130, 130, 5, 6, 6, 7, 130, 130, 130,
            1, 1, 1, 1, 1, 1, 1, 1, 1, 6, 6, 6, 6, 1, 1, 1,
            6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6,
            6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6,
            11, 11, 11, 11, 11, 11, 11, 11, 11, 11, 11, 11, 11, 11, 11, 11, 
            130, 130, 130, 130, 130, 130, 130, 130, 130, 130, 130, 130, 130, 130, 130, 130,
            130, 130, 130, 130, 130, 130, 130, 130, 130, 130, 130, 130, 130, 130, 130, 130,
            130, 130, 130, 130, 130, 130, 130, 130, 130, 130, 130, 130, 130, 130, 130, 130,
            130, 130, 130, 130, 130, 130, 130, 130, 130, 130, 130, 130, 130, 130, 130, 130,
            130, 130, 130, 130, 130, 130, 130, 130, 130, 130, 130, 130, 130, 130, 130, 130,
            130, 130, 130, 130, 130, 130, 130, 130, 130, 130, 130, 130, 130, 130, 130, 130,
            130, 130, 130, 130, 130, 130, 130, 130, 130, 130, 130, 130, 130, 130, 130, 130,
            130, 130, 130, 130, 130, 130, 130, 130, 130, 130, 130, 130, 130, 130, 130, 130,
            130, 130, 130, 130, 130, 130, 130, 130, 130, 130, 130, 130, 130, 130, 130, 130,

        ],
    );
    let lvl1map_4 = Tilemap::new(
        Vec2i(0, -256),
        (16, 16),
        &lvl1tileset,
        vec![
            16, 16, 16, 17, 130, 130, 15, 16, 16, 16, 16, 16, 16, 16, 16, 16,
            21, 21, 21, 22, 130, 130, 20, 21, 21, 21, 21, 21, 21, 21, 21, 21,
            26, 26, 26, 27, 130, 130, 25, 26, 26, 26, 26, 26, 26, 26, 26, 26,
            1, 1, 1, 2, 130, 130, 0, 1, 1, 1, 1, 1, 1, 1, 1, 1,
            6, 6, 6, 7, 130, 130, 5, 6, 6, 6, 6, 6, 6, 6, 6, 6,
            6, 6, 6, 7, 130, 130, 5, 6, 6, 6, 6, 6, 6, 6, 6, 6,
            11, 11, 11, 12,  130, 130,  5, 6, 6, 6, 6, 11, 11, 11, 11, 11,
            130, 130, 130, 130, 130, 130, 5, 6, 6, 6, 7, 130, 130, 130, 130, 130,
            130, 130, 130, 130, 130, 130, 5, 6, 6, 6, 7, 130, 130, 130, 130, 130, 
            130, 130, 130, 130, 130, 130, 5, 6, 6, 6, 6, 1, 1, 1, 1, 1, 
            130, 130, 130, 130, 130, 130, 5, 6, 6, 6, 6, 6, 6, 6, 6, 6, 
            130, 130, 130, 130, 130, 130, 5, 6, 6, 6, 6, 6, 6, 6, 6, 6,
            130, 130, 130, 130, 130, 130, 5, 6, 6, 6, 6, 11, 11, 11, 11, 11,
            130, 130, 130, 130, 130, 130, 5, 6, 6, 6, 7, 130, 130, 130, 130, 130,
            130, 130, 130, 130, 130, 130, 5, 6, 6, 6, 7, 130, 130, 130, 130, 130,
            130, 130, 130, 130, 130, 130, 5, 6, 6, 6, 7, 130, 130, 130, 130, 130,

        ],
    );
    let lvl1map_5 = Tilemap::new(
        Vec2i(256, -256),
        (16, 16),
        &lvl1tileset,
        //40, 41, 42, 43,
        //45, 46, 47, 48,
        //50, 51, 52, 53,
        vec![
            16, 16, 16, 17, 130, 130, 130, 130, 130, 130, 130, 15, 16, 16, 16, 17, 
            21, 21, 21, 22, 130, 130, 130, 130, 130, 130, 130, 20, 21, 21, 21, 22,
            26, 26, 26, 27, 130, 130, 130, 130, 130, 130, 130, 25, 26, 26, 26, 27,
            1, 1, 1, 2, 130, 130, 130, 130, 130, 130, 130, 0, 1, 1, 1, 1,
            6, 6, 6, 7, 130, 130, 130, 130, 130, 130, 130, 5, 6, 6, 6, 6,
            6, 6, 6, 7, 130, 130, 130, 130, 130, 130, 130, 5, 6, 6, 6, 6,
            11, 11, 11, 12, 130, 130, 130, 130, 130, 130, 130, 5, 6, 6, 11, 11,
            130, 130, 130, 130, 130, 130, 130, 130, 130, 130, 130, 5, 6, 7, 130, 130,  
            130, 130, 130, 130, 130, 130, 130, 130, 130, 130, 130, 5, 6, 7, 130, 130,
            1, 1, 1, 1, 1, 1, 1, 2, 130, 130, 130, 5, 6, 7, 130, 130, 
            6, 6, 6, 6, 6, 6, 6, 7, 130, 130, 130, 5, 6, 7, 130, 130, 
            6, 6, 6, 6, 6, 6, 6, 7, 130, 130, 130, 5, 6, 7, 130, 130,
            11, 11, 11, 6, 6, 6, 6, 7, 130, 130, 130, 5, 6, 7, 130, 130,
            130, 130, 130, 5, 6, 6, 6, 7, 130, 130, 130, 5, 6, 7, 130, 130,
            130, 130, 130, 5, 6, 6, 6, 7, 130, 130, 130, 5, 6, 7, 130, 130,
            130, 130, 130, 5, 6, 6, 6, 7, 130, 130, 130, 5, 6, 7, 130, 130,

        ],
    );
    let lvl1map_6 = Tilemap::new(
        Vec2i(-256, -256),
        (16, 16),
        &lvl1tileset,
        vec![
            130, 130, 130, 130, 130, 130, 130, 130, 130, 15, 16, 16, 16, 16, 16, 16,
            130, 130, 130, 130, 130, 130, 130, 130, 130, 20, 21, 21, 21, 21, 21, 21,
            130, 130, 130, 130, 130, 130, 130, 130, 130, 25, 26, 26, 26, 26, 26, 26,
            130, 130, 130, 130, 130, 130, 130, 130, 130, 0, 1, 1, 1, 1, 1, 1,
            130, 130, 130, 130, 130, 130, 130, 130, 130, 5, 6, 6, 6, 6, 6, 6,
            130, 130, 130, 130, 130, 130, 130, 130, 130, 5, 6, 6, 6, 6, 6, 6,
            130, 130, 130, 130, 130, 130, 130, 130, 130, 10, 11, 11, 11, 11, 11, 11,
            130, 130, 130, 130, 130, 130, 130, 130, 130, 5, 6, 6, 7, 130, 130, 130,  
            130, 130, 130, 130, 130, 130, 130, 130, 130, 5, 6, 6, 7, 130, 130, 130,
            130, 130, 130, 130, 130, 130, 130, 130, 130, 5, 6, 6, 7, 130, 130, 130, 
            130, 130, 130, 130, 130, 130, 130, 130, 130, 5, 6, 6, 7, 130, 130, 130, 
            130, 130, 130, 130, 130, 130, 130, 130, 130, 5, 6, 6, 7, 130, 130, 130,
            130, 130, 130, 130, 130, 130, 130, 130, 130, 5, 6, 6, 7, 130, 130, 130,
            130, 130, 130, 130, 130, 130, 130, 130, 130, 5, 6, 6, 7, 130, 130, 130,
            130, 130, 130, 130, 130, 130, 130, 130, 130, 5, 6, 6, 7, 130, 130, 130,
            130, 130, 130, 130, 130, 130, 130, 130, 130, 5, 6, 6, 7, 130, 130, 130,

        ],
    );
    let end_screen = Tilemap::new(
        Vec2i(0,0),
        // Map size
        (16, 16),
        &tileset1,
        // Tile grid
        vec![
            0, 1, 0, 1, 0, 1, 0, 1, 0, 1, 0, 1, 0, 1, 0, 1,
            2, 3, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 
            1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1,
            1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 
            1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1,
            1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1,
            1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1,
            1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 
            1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1,
            1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 
            1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1,
            1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 
            1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1,
            1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 
            1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1,
            1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 
        ]
    );

    let lvl2_map1 = Tilemap::new(
        Vec2i(0, 0),
        (16, 16),
        &lvl2tileset,
        vec![
            51, 1, 0, 1, 0, 1, 0, 1, 0, 1, 0, 1, 0, 1, 0, 1,
            54, 3, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 
            57, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1,
            1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 
            1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1,
            1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1,
            1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1,
            1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 
            1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1,
            1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 
            1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1,
            1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 
            1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1,
            1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 
            1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1,
            1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 
        ]
    );
    // Here's our game rules (the engine doesn't know about these)
    let levels:Vec<Level> = vec![
        (vec![start_screen],   
       // Initial entities on level start
         vec![]
         ),

        (vec![lvl1map_1,lvl1map_2,lvl1map_3,lvl1map_4,lvl1map_5,lvl1map_6],
            // Initial entities on level start
         vec![
            (EntityType::Barrier, 17,-12),
            (EntityType::lvl1Exit, 27,-16),
            (EntityType::Enemy, 20, 20),
            (EntityType::Player, 8, 13),
            ]
        ), 

        (vec![end_screen],   
         vec![
                (EntityType::Player, 0, 0),
                (EntityType::Enemy, 10, 0)
            ]
        ),   
    ];
    let up_frames =vec![
        Rect {
        x: 0,
        y: 90,
        w: 16,
        h: 32,
    }, Rect {
        x: 0,
        y: 90,
        w: 32,
        h: 32,
    },
    Rect {
        x: 0,
        y: 90,
        w: 48,
        h: 32,
    },
    Rect {
        x: 0,
        y: 90,
        w: 64,
        h: 32,
    }
    ];
    let barrier_touched=vec![
        Rect {
            x: 0,
            y: 0,
            w: 32,
            h: 32,
        }, Rect {
            x: 32,
            y: 0,
            w: 32,
            h: 32,
        },
    ];
    let barrier_tex = rsrc.load_texture(Path::new("content/barrier.png"));
    let barrier_anim = Rc::new(Animation::freeze(Rect{x:0,y:0,w:32,h:32}));
    let player_tex = rsrc.load_texture(Path::new("content/player.png"));
    let player_anim = Rc::new(Animation::freeze(Rect{x:0,y:0,w:16,h:32}));
    let player_up_anim = Rc::new(Animation {
        frames: vec![(Rect{x:0,y:90,w:16,h:32}, 1)],
        looping: false,
    });
    let enemy_tex = Rc::clone(&player_tex);
    let enemy_anim = Rc::new(Animation::freeze(Rect{x:0,y:0,w:16,h:32}));
    let lvl1exit_tex = rsrc.load_texture(Path::new("content/lvl1exit.png"));
    let lvl1exit_anim = Rc::new(Animation::freeze(Rect{x:0,y:0,w:64,h:48}));

    // And here's our game state, which is just stuff that changes.
    // We'll say an entity is a type, a position, a velocity, a size, a texture, and an animation state.
    // State here will stitch them all together.
    let mut game = GameState{
        // Every entity has a position, a size, a texture, and animation state.
        // Assume entity 0 is the player
        types: vec![
            // In a real example we'd provide nicer accessors than this
            levels[1].1[0].0,
            levels[1].1[1].0,
            levels[1].1[2].0,
            levels[1].1[3].0,
        ],
        positions: vec![
            Vec2i(
                levels[1].1[0].1 * 16,
                levels[1].1[0].2 * 16,
            ),
            Vec2i(
                levels[1].1[1].1 * 16,
                levels[1].1[1].2 * 16,
            ),
            Vec2i(
                levels[1].1[2].1 * 16,
                levels[1].1[2].2 * 16,
            ),
            Vec2i(
                levels[1].1[3].1 * 16,
                levels[1].1[3].2 * 16,
            )
        ],
        velocities: vec![Vec2i(0,0), Vec2i(0,0),Vec2i(0,0), Vec2i(0,0)],
        sizes: vec![(16,16), (16,16),(16,16), (16,16)],
        // Could be texture handles instead, let's talk about that in two weeks
        textures: vec![Rc::clone(&barrier_tex),
                       Rc::clone(&lvl1exit_tex),
                       Rc::clone(&enemy_tex),
                       Rc::clone(&player_tex),],
        anim_state: vec![barrier_anim.start(),lvl1exit_anim.start(),player_anim.start(),enemy_anim.start()],
        // Current level
        level: 0,
        // Camera position
        camera: Vec2i(0, 0),
        mode:Mode::Title,
        movable:true,
        direction:Direction::Up
    };
    
    engine2d::run(WIDTH, HEIGHT, window_builder, rsrc, levels, game, draw_game, update_game);
}

fn draw_game(resources:&Resources, levels: &Vec<Level>, state: &GameState, screen: &mut Screen, frame:usize) {
    screen.clear(Rgba(80, 80, 80, 255));
    screen.set_scroll(state.camera);
    // levels[state.level].0.draw(screen);
    // for t in levels[state.level].0.iter(){
    //     t.draw(screen);
    // }
    // for ((pos,tex),anim) in state.positions.iter().zip(state.textures.iter()).zip(state.anim_state.iter()) {
    //     screen.bitblt(tex,anim.frame(),*pos);
    // }
    state.mode.display(&state, screen,levels);
}

fn update_game(resources:&Resources, levels: &Vec<Level>, state: &mut GameState, input: &WinitInputHelper, frame: usize) {
    // Determine enemy velocity

    state.mode = state.mode.update(state, input,levels);
    // Detect collisions: Convert positions and sizes to collision bodies, generate contacts

    // Handle collisions: Apply restitution impulses.
    //contacts.clear();
    // let contacts = engine2d::collision::gather_contacts(&state.positions, &state.sizes);
    // let mut tile_contacts= engine2d::collision::gather_contacts_tilemap(&state.positions, &state.sizes,&levels[1].0);
    // for contact in contacts.iter(){
    //     println!("{}",state.level);
    //     match (levels[state.level].1[contact.a].0, levels[state.level].1[contact.b].0){
    //         (EntityType::Player, EntityType::Barrier) => {
    //             state.movable = false;
    //             //Generate text on screen
    //             let barrier_touched_anim = Rc::new(Animation {
    //                 frames: vec![(Rect{x:0,y:0,w:32,h:32}, 1),
    //                              (Rect {x: 32,y: 0,w: 32, h: 32},1)],
    //                 looping: false,
    //             });
    //             barrier_touched_anim.start();
                
    //         }
    //         (EntityType::Player, EntityType::lvl1Exit) => {
    //             state.movable = false;
    //             // state.level=2;
    //             Mode::Lvl2;
    //         }
    //         _ => {}
    //     }       
    // }
    // engine2d::collision::restitute(&mut state.positions, &mut state.sizes,&mut tile_contacts);
    // Update game rules: What happens when the player touches things?  When enemies touch walls?  Etc.
    // Maybe scroll the camera or change level
}
