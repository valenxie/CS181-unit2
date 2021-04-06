use core::time;
use std::{fs::File, io::BufReader, path::Path, time::Duration};
use std::rc::Rc;

use winit::window::WindowBuilder;
use winit::dpi::LogicalSize;
use winit::event::VirtualKeyCode;
use winit_input_helper::WinitInputHelper;

use engine2d::{animation, texture, types::*};
use engine2d::graphics::Screen;
use engine2d::tiles::*;
use engine2d::animation::*;
use engine2d::text::{self, DrawText};
use engine2d::sprite::*;

use engine2d::collision::*;
// Imagine a Resources struct (we'll call it AssetDB or Assets in the future)
// which wraps all accesses to textures, sounds, animations, etc.
use engine2d::resources::*;
use engine2d::texture::Texture;
use rodio::{Decoder, OutputStream, Sink};
use rodio::source::{SineWave, Source};


const WIDTH: usize = 256;
const HEIGHT: usize = 256;

#[derive(Clone,Copy,PartialEq,Eq,Debug)]
enum EntityType {
    Player,
    Enemy,
    Barrier,
    lvl1Exit,
    lvl2Exit,
    lvl2Entrance,
    Bridge

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
                        game.anim_state[3].change_time(3);
                        if game.positions[3].0 < 1000 {
                            game.velocities[3].0 = 1;
                        } else {
                            game.velocities[3].0 = 0;
                        }
                        if game.positions[3].0 >= game.camera.0+16*5 && 
                            game.camera.0 < 256 {
                            game.camera.0 += 1;
                        }
                    } else if input.key_held(VirtualKeyCode::Left) {
                        game.anim_state[3].change_time(2);
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
                        game.anim_state[3].change_time(0);
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
                        game.anim_state[3].change_time(1);
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
                if input.key_held(VirtualKeyCode::Q){
                    Mode::EndGame
                } else if input.key_held(VirtualKeyCode::T){
                    Mode::Lvl2
                }else {
                    self
                }

            },
            Mode::Lvl2 => {
                if game.movable {                    
                    if input.key_held(VirtualKeyCode::Right) {
                        if game.positions[3].0 < 2000 {
                            game.anim_state[3].change_time(3);
                            game.velocities[3].0 = 1;
                        } else {
                            game.velocities[3].0 = 0;
                        }
                        if game.positions[3].0 >= game.camera.0+16*5 && 
                            game.camera.0 < 512 {
                            game.camera.0 += 1;
                        }
                    } else if input.key_held(VirtualKeyCode::Left) {
                        game.anim_state[3].change_time(2);
                        if game.positions[3].0 > 48
                         {
                            game.velocities[3].0 = -1;
                        } else {
                            game.velocities[3].0 = 0;
                        }
                        if game.positions[3].0 <= game.camera.0+16*5 && 
                            game.camera.0 > 0 {
                            game.camera.0 -= 1;
                        }
                    } else {
                        game.velocities[3].0 = 0;
                    }
                    if input.key_held(VirtualKeyCode::Up) {
                        game.anim_state[3].change_time(0);
                        if game.positions[3].1 > 0 {
                            game.velocities[3].1 = -1;
                        } else {
                            game.velocities[3].1 = 0;
                        }
                        if game.positions[3].1 <= game.camera.1+16*5 && 
                            game.camera.1 > 0 {
                            game.camera.1 -= 1;
                        }
                    } else if input.key_held(VirtualKeyCode::Down) {
                        game.anim_state[3].change_time(1);
                        if game.positions[3].1 < 512 {
                            game.velocities[3].1 = 1;
                        } else {
                            game.velocities[3].1 = 0;
                        }
                    
                        if game.positions[3].1 >= game.camera.1+16*5 && 
                            game.camera.1 < 256 {
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
    fn display(&self, game:&GameState, screen: &mut Screen, levels: &Vec<Level>, rsrc:&Resources) {
        match self {
            Mode::Title => {
                // let rsrc = Resources::new();
    
                for t in levels[0].0.iter(){
                    t.draw(screen);
                }
                screen.draw_text(
                    "finding home",
                    Vec2i(80, 80),
                    &rsrc.text,
                );
                screen.draw_text(
                    "press p to begin",
                    Vec2i(65, 180),
                    &rsrc.text,
                );
                screen.draw_text(
                    "press q to exit",
                    Vec2i(65, 200),
                    &rsrc.text,
                );

                
            },
            Mode::Lvl1=> {
                screen.set_scroll(game.camera);
                for t in levels[1].0.iter(){
                    t.draw(screen);
                }
                for ((pos,tex),anim) in game.positions.iter().zip(game.textures.iter()).zip(game.anim_state.iter()) {
                    // screen.bitblt(tex,anim.frame(),*pos);
                    // anim.animate();
                    screen.bitblt(tex,anim.frame(),*pos);
                } 
                screen.draw_text(
                    "press r to restart",
                    Vec2i(100, -230),
                    &rsrc.text,
                );               
            },
            Mode::Lvl2=> {
                screen.set_scroll(game.camera);
                for t in levels[2].0.iter(){
                    t.draw(screen);
                }
    
                for ((mut pos,mut tex),mut anim) in game.positions.iter().zip(game.textures.iter()).zip(game.anim_state.iter()) {
                    screen.bitblt(tex,anim.frame(),*pos);
                }
            },
            Mode::EndGame => {
                for t in levels[3].0.iter(){
                    t.draw(screen);
                }
                screen.draw_text(
                    "you are home",
                    Vec2i(65, 180),
                    &rsrc.text,
                );
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
        soundstream: (rodio::OutputStream, rodio::OutputStreamHandle),
    }

fn main() {
    let window_builder = {
        let size = LogicalSize::new(WIDTH as f64, HEIGHT as f64);
        WindowBuilder::new()
            .with_title("FindingHome")
            .with_inner_size(size)
            .with_min_inner_size(size)
            .with_resizable(false)
    };
    // Here's our resources...
    let rsrc = Resources::new();
    let hometileset = Rc::new(Tileset::new(
        vec![
            Tile{solid:false}, Tile{solid:true}, Tile{solid:false}, Tile{solid:false}, Tile{solid:false},
            Tile{solid:false},
        ],
        &rsrc.load_texture(Path::new("content/home.png"))
    ));
    let lvl1tileset = Rc::new(Tileset::new(
        vec![
            Tile{solid:false}, Tile{solid:true}, Tile{solid:false}, Tile{solid:false}, Tile{solid:false},
            Tile{solid:true}, Tile{solid:false}, Tile{solid:true}, Tile{solid:false}, Tile{solid:false},
            Tile{solid:true}, Tile{solid:true}, Tile{solid:true}, Tile{solid:false}, Tile{solid:false},
            Tile{solid:false}, Tile{solid:false}, Tile{solid:false}, Tile{solid:false}, Tile{solid:false},
            Tile{solid:false}, Tile{solid:false}, Tile{solid:false}, Tile{solid:false}, Tile{solid:false}, 
            Tile{solid:false}, Tile{solid:false}, Tile{solid:false}, Tile{solid:false}, Tile{solid:false},
            Tile{solid:false}, Tile{solid:false}, Tile{solid:false}, Tile{solid:false}, Tile{solid:false},
            Tile{solid:false}, Tile{solid:false}, Tile{solid:false}, Tile{solid:false}, Tile{solid:false},
            Tile{solid:false}, Tile{solid:false}, Tile{solid:false}, Tile{solid:false}, Tile{solid:false},
            Tile{solid:false}, Tile{solid:false}, Tile{solid:false}, Tile{solid:false}, Tile{solid:false},
            Tile{solid:false}, Tile{solid:false}, Tile{solid:false}, Tile{solid:false}, Tile{solid:false},
            Tile{solid:false}, Tile{solid:false}, Tile{solid:false}, Tile{solid:false}, Tile{solid:false},
            Tile{solid:false}, Tile{solid:false}, Tile{solid:false}, Tile{solid:false}, Tile{solid:false},
            Tile{solid:false}, Tile{solid:false}, Tile{solid:false}, Tile{solid:false}, Tile{solid:false},
            Tile{solid:false}, Tile{solid:false}, Tile{solid:false}, Tile{solid:false}, Tile{solid:false},
            Tile{solid:false}, Tile{solid:false}, Tile{solid:false}, Tile{solid:false}, Tile{solid:false},
            Tile{solid:false}, Tile{solid:false}, Tile{solid:false}, Tile{solid:false}, Tile{solid:false},
            Tile{solid:false}, Tile{solid:false}, Tile{solid:false}, Tile{solid:false}, Tile{solid:false},
            Tile{solid:false}, Tile{solid:false}, Tile{solid:false}, Tile{solid:false}, Tile{solid:false},
            Tile{solid:false}, Tile{solid:false}, Tile{solid:false}, Tile{solid:false}, Tile{solid:false},
            Tile{solid:false}, Tile{solid:false}, Tile{solid:false}, Tile{solid:false}, Tile{solid:false},
            Tile{solid:false}, Tile{solid:false}, Tile{solid:false}, Tile{solid:false}, Tile{solid:false},
            Tile{solid:false}, Tile{solid:false}, Tile{solid:false}, Tile{solid:false}, Tile{solid:false},
            Tile{solid:false}, Tile{solid:false}, Tile{solid:false}, Tile{solid:false}, Tile{solid:false},
            Tile{solid:false}, Tile{solid:false}, Tile{solid:false}, Tile{solid:false}, Tile{solid:false},
            Tile{solid:false}, Tile{solid:false}, Tile{solid:false}, Tile{solid:false}, Tile{solid:false},
            Tile{solid:false},
        ],
        &rsrc.load_texture(Path::new("content/lvl1.png"))
    ));
    let lvl2tileset = Rc::new(Tileset::new(
        vec![
            Tile{solid:false}, Tile{solid:false}, Tile{solid:false}, Tile{solid:false}, Tile{solid:false},
            Tile{solid:false}, Tile{solid:false}, Tile{solid:false}, Tile{solid:false}, Tile{solid:false},//5
            Tile{solid:false}, Tile{solid:false}, Tile{solid:false}, Tile{solid:false}, Tile{solid:false},
            Tile{solid:false}, Tile{solid:false}, Tile{solid:false}, Tile{solid:false}, Tile{solid:false},//15
            Tile{solid:false}, Tile{solid:false}, Tile{solid:false}, Tile{solid:false}, Tile{solid:false},
            Tile{solid:false}, Tile{solid:false}, Tile{solid:false}, Tile{solid:false}, Tile{solid:false},//25
            Tile{solid:false}, Tile{solid:false}, Tile{solid:false}, Tile{solid:false}, Tile{solid:false},
            Tile{solid:false}, Tile{solid:false}, Tile{solid:false}, Tile{solid:false}, Tile{solid:false},//35
            Tile{solid:false}, Tile{solid:false}, Tile{solid:false}, Tile{solid:false}, Tile{solid:false},
            Tile{solid:false}, Tile{solid:false}, Tile{solid:false}, Tile{solid:false}, Tile{solid:false},//45
            Tile{solid:false}, Tile{solid:false}, Tile{solid:true}, Tile{solid:false}, Tile{solid:false},//50
            Tile{solid:false}, Tile{solid:true}, Tile{solid:false}, Tile{solid:false}, Tile{solid:true},//55
            Tile{solid:true}, Tile{solid:true}, Tile{solid:true}, Tile{solid:true}, Tile{solid:true},
            Tile{solid:true}, Tile{solid:true}, Tile{solid:true}, Tile{solid:true},
            
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
            1, 1, 1, 2, 130, 130, 130, 130, 130, 130, 130, 0, 6, 6, 1, 1,
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
        &hometileset,
        // Tile grid
        vec![
            0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 
            0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            2, 3, 2, 3, 2, 3, 2, 3, 2, 3, 2, 3, 2, 3, 2, 3,
            4, 5, 4, 5, 4, 5, 4, 5, 4, 5, 4, 5, 4, 5, 4, 5, 
            2, 3, 2, 3, 2, 3, 2, 3, 2, 3, 2, 3, 2, 3, 2, 3,
            4, 5, 4, 5, 4, 5, 4, 5, 4, 5, 4, 5, 4, 5, 4, 5,  
        ]
    );

    let lvl2_map1 = Tilemap::new(
        Vec2i(0, 0),
        (16, 16),
        &lvl2tileset,
        vec![
            //52:solid
            42, 43, 43, 43, 43, 43, 43, 43, 43, 43, 43, 43, 43, 43, 43, 43,
            45, 46, 46, 46, 46, 46, 46, 46, 46, 46, 46, 46, 46, 46, 46, 46, 
            48, 49, 49, 49, 49, 49, 49, 49, 49, 49, 49, 49, 49, 49, 49, 49,
            51, 52, 52, 55, 55, 55, 55, 55, 55, 55, 55, 55, 55, 55, 55, 55, 
            54, 55, 52, 55, 55, 55, 55, 55, 55, 55, 55, 55, 55, 55, 55, 55,
            57, 58, 52, 55, 55, 55, 57, 58, 58, 58, 58, 58, 58, 58, 58, 58,
            60, 61, 62, 15, 16, 16, 60, 61, 61, 61, 61, 61, 61, 61, 61, 62,
            63, 64, 65, 18, 19, 19, 63, 64, 64, 64, 64, 64, 64, 64, 64, 65,
            66, 67, 68, 18, 19, 19, 66, 67, 67, 67, 67, 67, 67, 67, 67, 68,
            52, 52, 52, 12, 12, 12, 52, 52, 52, 52, 52, 52, 52, 52, 52, 52, 
            52, 52, 52, 12, 12, 12, 52, 52, 52, 52, 52, 52, 52, 52, 52, 52, 
            52, 52, 52, 12, 12, 12, 52, 52, 52, 52, 52, 52, 52, 52, 52, 52, 
            52, 52, 52, 12, 12, 12, 52, 52, 52, 52, 52, 52, 52, 52, 52, 52, 
            52, 52, 52, 12, 12, 12, 52, 52, 52, 52, 52, 52, 52, 52, 52, 52, 
            52, 52, 52, 12, 12, 12, 52, 52, 52, 52, 52, 52, 52, 52, 52, 52, 
            52, 52, 52, 12, 12, 12, 12, 12, 12, 12, 12, 12, 12, 12, 12, 12, 
        ]
    );

    let lvl2_map2 = Tilemap::new(
        Vec2i(256, 0),
        (16, 16),
        &lvl2tileset,
        vec![
            //52:solid
            43, 43, 43, 43, 43, 43, 43, 43, 43, 43, 43, 43, 43, 44, 37, 37,
            46, 46, 46, 46, 46, 46, 46, 46, 46, 46, 46, 46, 46, 47, 37, 37,
            49, 49, 49, 49, 49, 49, 49, 49, 49, 49, 49, 49, 49, 50, 37, 37,
            55, 55, 55, 55, 55, 55, 55, 55, 55, 55, 55, 55, 55, 56, 40, 40, 
            55, 55, 55, 55, 55, 55, 55, 55, 55, 55, 55, 55, 55, 56, 55, 55,
            55, 55, 55, 58, 58, 21, 22, 58, 58, 58, 58, 55, 55, 59, 52, 52,
            15, 16, 16, 60, 61, 24, 25, 61, 61, 61, 61, 15, 16, 62, 52, 52,
            18, 19, 19, 63, 64, 27, 28, 64, 64, 64, 64, 18, 19, 65, 52, 52,
            18, 19, 19, 66, 67, 0, 1, 67, 67, 67, 67, 18, 19, 68, 52, 52,
            12, 12, 12, 52, 36, 37, 37, 37, 38, 52, 52, 12, 12, 12, 12, 52, 
            12, 12, 12, 52, 36, 37, 37, 37, 38, 52, 52, 12, 12, 12, 12, 52, 
            12, 12, 12, 52, 36, 37, 37, 37, 38, 52, 52, 12, 12, 52, 52, 52, 
            12, 12, 12, 52, 36, 37, 37, 37, 38, 52, 52, 12, 12, 52, 52, 52, 
            12, 12, 12, 52, 36, 37, 37, 37, 38, 52, 52, 12, 12, 52, 52, 52, 
            12, 12, 12, 52, 36, 37, 37, 37, 38, 52, 52, 12, 12, 12, 12, 12, 
            12, 12, 12, 12, 36, 37, 37, 37, 38, 12, 12, 12, 12, 12, 12, 12,
        ]
    );
    let lvl2_map3 = Tilemap::new(
        Vec2i(512, 0),
        (16, 16),
        &lvl2tileset,
        vec![
            //52:solid
            37, 37, 37, 37, 37, 37, 37, 37, 37, 37, 37, 37, 37, 37, 37, 37,
            37, 37, 37, 37, 37, 37, 37, 37, 37, 37, 37, 37, 37, 37, 37, 37,
            37, 37, 37, 37, 37, 37, 37, 37, 37, 37, 37, 37, 37, 37, 37, 37,
            40, 40, 40, 40, 40, 40, 40, 40, 40, 40, 40, 40, 40, 40, 40, 40, 
            54, 55, 55, 55, 55, 55, 55, 55, 55, 55, 55, 55, 55, 55, 55, 55,
            57, 55, 55, 55, 58, 58, 58, 58, 58, 58, 58, 58, 58, 55, 55, 58,
            60, 15, 16, 16, 60, 61, 61, 61, 61, 61, 61, 61, 61, 15, 16, 62,
            63, 18, 19, 19, 63, 64, 64, 64, 64, 64, 64, 64, 64, 18, 19, 65,
            66, 18, 19, 19, 66, 67, 67, 67, 67, 67, 67, 67, 67, 18, 19, 66,
            52, 12, 12, 12, 12, 12, 12, 12, 12, 12, 12, 12, 12, 12, 12, 52,
            52, 12, 12, 12, 12, 12, 12, 12, 12, 12, 12, 12, 12, 12, 12, 52, 
            52, 52, 52, 52, 52, 52, 52, 52, 52, 52, 52, 52, 52, 12, 12, 52, 
            52, 52, 33, 34, 34, 34, 34, 34, 34, 34, 34, 35, 52, 12, 12, 52, 
            52, 52, 36, 37, 37, 37, 37, 37, 37, 37, 37, 38, 52, 12, 12, 52, 
            12, 52, 36, 37, 37, 37, 37, 37, 37, 37, 37, 38, 52, 12, 12, 52, 
            12, 52, 36, 37, 37, 37, 37, 37, 37, 37, 37, 38, 52, 12, 12, 52,
        ]
    );
    let lvl2_map4 = Tilemap::new(
        Vec2i(0, 256),
        (16, 16),
        &lvl2tileset,
        vec![
            //52:solid
            52, 52, 52, 12, 12, 12, 12, 12, 12, 12, 12, 12, 12, 12, 12, 12,
            52, 52, 52, 12, 12, 12, 12, 12, 12, 12, 12, 12, 12, 12, 12, 12,
            52, 52, 52, 12, 12, 12, 52, 52, 52, 52, 52, 52, 52, 52, 52, 52,
            52, 52, 52, 12, 12, 12, 52, 52, 52, 52, 52, 52, 52, 52, 52, 52,
            52, 52, 52, 12, 12, 12, 52, 52, 52, 52, 52, 52, 52, 52, 52, 52,
            12, 12, 12, 12, 12, 12, 52, 52, 52, 52, 52, 52, 52, 52, 52, 52,
            12, 12, 12, 12, 12, 12, 52, 52, 52, 52, 52, 52, 52, 52, 52, 52,
            12, 12, 12, 12, 12, 12, 52, 52, 52, 52, 52, 52, 52, 52, 52, 52,
            52, 52, 52, 12, 12, 12, 52, 52, 52, 52, 52, 52, 52, 52, 52, 52,
            52, 52, 52, 12, 12, 12, 52, 52, 52, 52, 52, 52, 52, 52, 52, 52,
            52, 52, 52, 12, 12, 12, 52, 52, 52, 52, 52, 52, 52, 52, 52, 52, 
            52, 52, 52, 12, 12, 12, 52, 52, 52, 52, 52, 52, 52, 52, 52, 52, 
            52, 52, 52, 12, 12, 12, 52, 52, 52, 52, 52, 52, 52, 52, 52, 52, 
            52, 52, 52, 12, 12, 12, 52, 52, 52, 52, 52, 52, 52, 52, 52, 52,
            52, 52, 52, 12, 12, 12, 52, 52, 52, 52, 52, 52, 52, 52, 52, 52,
            52, 52, 52, 12, 12, 12, 52, 52, 52, 52, 52, 52, 52, 52, 52, 52,
        ]
    );
    let lvl2_map5 = Tilemap::new(
        Vec2i(256, 256),
        (16, 16),
        &lvl2tileset,
        vec![
            //52:solid
            12, 12, 12, 12, 36, 37, 37, 37, 38, 12, 12, 12, 12, 52, 52, 52,
            12, 12, 12, 12, 36, 37, 37, 37, 38, 12, 12, 12, 12, 52, 52, 52,  
            12, 12, 12, 52, 36, 37, 37, 37, 38, 52, 52, 12, 12, 52, 52, 52, 
            12, 12, 12, 52, 36, 37, 37, 37, 38, 52, 52, 12, 12, 52, 52, 52, 
            12, 12, 12, 52, 36, 37, 37, 37, 38, 52, 52, 12, 12, 52, 52, 52,
            12, 12, 12, 52, 36, 37, 37, 37, 38, 52, 52, 12, 12, 52, 52, 52,
            12, 12, 12, 52, 36, 37, 37, 37, 38, 52, 52, 12, 12, 12, 12, 12, 
            12, 12, 12, 52, 36, 37, 37, 37, 38, 52, 52, 12, 12, 12, 12, 12,  
            12, 12, 12, 52, 36, 37, 37, 37, 38, 52, 52, 12, 12, 52, 52, 52, 
            12, 12, 12, 52, 36, 37, 37, 37, 38, 52, 52, 12, 12, 52, 52, 52, 
            12, 12, 12, 52, 36, 37, 37, 37, 38, 52, 52, 12, 12, 52, 52, 52, 
            12, 12, 12, 52, 36, 37, 37, 37, 38, 52, 52, 12, 12, 52, 52, 52,
            12, 12, 12, 52, 36, 37, 37, 37, 38, 52, 52, 12, 12, 52, 52, 52, 
            12, 12, 12, 52, 36, 37, 37, 37, 38, 52, 52, 12, 12, 52, 52, 52,
            12, 12, 12, 52, 36, 37, 37, 37, 38, 52, 52, 12, 12, 12, 12, 12,
            12, 12, 12, 52, 36, 37, 37, 37, 38, 52, 52, 12, 12, 12, 12, 12,
        ]
    );
    let lvl2_map6 = Tilemap::new(
        Vec2i(512, 256),
        (16, 16),
        &lvl2tileset,
        vec![
            //52:solid
            52, 52, 36, 37, 37, 37, 37, 37, 37, 37, 37, 38, 52, 12, 12, 52,
            52, 52, 36, 37, 37, 37, 37, 37, 37, 37, 37, 38, 52, 12, 12, 52,
            52, 52, 36, 37, 37, 37, 37, 37, 37, 37, 37, 38, 52, 12, 12, 52,
            52, 52, 39, 40, 40, 40, 40, 40, 40, 40, 40, 41, 52, 12, 12, 52, 
            52, 52, 52, 52, 52, 52, 52, 52, 52, 52, 52, 52, 52, 12, 12, 52,
            52, 52, 52, 52, 52, 52, 52, 52, 52, 52, 52, 52, 52, 12, 12, 52,
            12, 12, 12, 12, 12, 12, 12, 12, 12, 12, 12, 52, 52, 12, 12, 52,
            12, 12, 12, 12, 12, 12, 12, 12, 12, 12, 12, 52, 52, 12, 12, 52,
            12, 12, 52, 52, 52, 52, 52, 52, 52, 12, 12, 52, 52, 12, 12, 52,
            12, 12, 12, 12, 12, 12, 12, 12, 52, 12, 12, 52, 52, 12, 12, 52,
            12, 12, 12, 12, 12, 12, 12, 12, 52, 12, 12, 52, 52, 12, 12, 52, 
            52, 52, 52, 52, 52, 52, 12, 12, 52, 12, 12, 52, 52, 12, 12, 52, 
            12, 12, 12, 12, 12, 12, 12, 12, 52, 52, 52, 52, 52, 12, 12, 52, 
            52, 52, 52, 52, 52, 52, 52, 52, 52, 52, 52, 52, 52, 12, 12, 52, 
            12, 12, 12, 12, 12, 12, 12, 12, 12, 12, 12, 12, 12, 12, 12, 52, 
            12, 12, 12, 12, 12, 12, 12, 12, 12, 12, 12, 12, 12, 12, 12, 52,
        ]
    );
    // Here's our game rules (the engine doesn't know about these)
    let levels:Vec<Level> = vec![
        (vec![start_screen],   
         vec![]
         ),

        (vec![lvl1map_1,lvl1map_2,lvl1map_3,lvl1map_4,lvl1map_5,lvl1map_6],
         vec![
            (EntityType::Barrier, 17,-12),
            (EntityType::lvl1Exit, 27,-16),
            (EntityType::Enemy, 20, 20),
            (EntityType::Player, 8, 13),
            ]
        ), 
        (vec![lvl2_map1,lvl2_map2,lvl2_map3,lvl2_map4,lvl2_map5,lvl2_map6],  
              vec![
                (EntityType::lvl2Entrance, 45, 10),
                (EntityType::lvl2Exit, 650,10),
                (EntityType::Bridge, 320,220),
                (EntityType::Player, 45, 15)
              ]
            
        ),

        (vec![end_screen],   
         vec![]
        ),   
    ];

    let barrier_tex = rsrc.load_texture(Path::new("content/barrier.png"));
    // let barrier_anim = Rc::new(Animation::freeze(Rect{x:0,y:0,w:32,h:32}));
    let player_tex = rsrc.load_texture(Path::new("content/player.png"));
    let enemy_tex = Rc::clone(&player_tex);
    let enemy_anim = Rc::new(Animation::freeze(Rect{x:0,y:0,w:16,h:32}));
    let lvl1exit_tex = rsrc.load_texture(Path::new("content/lvl1exit.png"));
    let lvl1exit_anim = Rc::new(Animation::freeze(Rect{x:0,y:0,w:64,h:48}));
    let barrier_anim = Rc::new(Animation {
        frames: vec![(Rect{x:0,y:0,w:32,h:32}, 0),
                     (Rect {x: 32,y: 0,w: 32, h: 32},1)],
        looping: false,
    }); 
    let player_anim = Rc::new(Animation {
        frames: vec![(Rect{x:0,y:64,w:16,h:32},0),//(Rect{x:16,y:64,w:16,h:32},1),
                     (Rect{x:0,y:0,w:16,h:32},1),
                     (Rect{x:0,y:32,w:16,h:32},2),
                     (Rect{x:0,y:96,w:16,h:32},3)],
        looping: false,
    });

    // And here's our game state, which is just stuff that changes.
    // We'll say an entity is a type, a position, a velocity, a size, a texture, and an animation state.
    // State here will stitch them all together.
    let game = GameState{
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
            ),
        ],
        velocities: vec![Vec2i(0,0), Vec2i(0,0),Vec2i(0,0), Vec2i(0,0),Vec2i(0,0),Vec2i(0,0), Vec2i(0,0)],
        sizes: vec![(16,16), (40,26),(16,16), (16,16),(16,16), (16,16),(16,16)],
        // Could be texture handles instead, let's talk about that in two weeks
        textures: vec![Rc::clone(&barrier_tex),
                       Rc::clone(&lvl1exit_tex),
                       Rc::clone(&enemy_tex),
                       Rc::clone(&player_tex),],
                    
        anim_state: vec![barrier_anim.start(),lvl1exit_anim.start(),barrier_anim.start(),player_anim.start()],
        // Current level
        level: 0,
        // Camera position
        camera: Vec2i(0, 0),
        mode:Mode::Title,
        movable:true,
        soundstream: OutputStream::try_default().unwrap()
    };
    // let file = BufReader::new(File::open("content/bgm.mp3").unwrap());
    // let source = Decoder::new(file).unwrap().delay(std::time::Duration::from_secs(5)).repeat_infinite();
    // game.soundstream.1.play_raw(source.convert_samples()).unwrap();

    engine2d::run(WIDTH, HEIGHT, window_builder, rsrc, levels, game, draw_game, update_game);
}

fn draw_game(resources:&Resources, levels: &Vec<Level>, state: &GameState, screen: &mut Screen, frame:usize) {
    screen.clear(Rgba(80, 80, 80, 255));
    screen.set_scroll(state.camera);
    state.mode.display(state, screen,levels,resources);
}

fn update_game(resources:&Resources, levels: &Vec<Level>, state: &mut GameState, input: &WinitInputHelper, frame: usize) {
    // Determine enemy velocity
    state.mode = state.mode.update(state, input,levels);
    // Detect collisions: Convert positions and sizes to collision bodies, generate contacts

    // Handle collisions: Apply restitution impulses.
    //contacts.clear();
    let contacts = engine2d::collision::gather_contacts(&state.positions, &state.sizes);
    for contact in contacts.iter(){
        // println!("{}",state.level);
        // println!("before{:?}",state.positions.len());
        match (state.types[contact.a],state.types[contact.b]){
            (EntityType::Player, EntityType::Barrier) => {
                state.movable = false;
                state.anim_state[0].change_time(1);

            }
            (EntityType::Player, EntityType::lvl1Exit) => {
                // state.positions[3]=Vec2i(20,20);
                state.camera=Vec2i(0,0);
                state.types.clear();
                state.velocities.clear();
                state.textures.clear();
                state.sizes.clear();
                state.positions.clear();
                state.anim_state.clear();
                // println!("after{:?}",state.positions.len());
                for (e_type,x,y) in levels[2].1.iter(){
                    state.types.push(*e_type);
                    state.positions.push(Vec2i(*x,*y));
                    state.velocities.push(Vec2i(0,0));
                    match e_type {
                        EntityType::Enemy => {}
                        EntityType::Barrier => {}
                        EntityType::lvl1Exit => {}
                        EntityType::lvl2Exit => {
                            state.sizes.push((80,80));
                            state.textures.push(resources.load_texture(Path::new("content/lvl2exit.png")));
                            state.anim_state.push(Rc::new(Animation::freeze(Rect{x:0,y:0,w:80,h:80})).start());
                            println!("{}",state.level);
                        }
                        EntityType::lvl2Entrance => {
                            state.sizes.push((32,32));
                            state.textures.push(resources.load_texture(Path::new("content/lvl2entrance.png")));
                            state.anim_state.push(Rc::new(Animation::freeze(Rect{x:0,y:0,w:64,h:64})).start())
                        }
                        EntityType::Bridge => {
                            state.sizes.push((16,16));
                            state.textures.push(resources.load_texture(Path::new("content/bridge.png")));
                            state.anim_state.push(Rc::new(Animation::freeze(Rect{x:0,y:0,w:80,h:112})).start())
                        }
                        EntityType::Player => {
                            state.sizes.push((16,16));
                            state.textures.push(resources.load_texture(Path::new("content/player.png")));
                            state.anim_state.push(Rc::new(Animation {
                                frames: vec![(Rect{x:0,y:64,w:16,h:32},0),//(Rect{x:16,y:64,w:16,h:32},1),
                                             (Rect{x:0,y:0,w:16,h:32},1),
                                             (Rect{x:0,y:32,w:16,h:32},2),
                                             (Rect{x:0,y:96,w:16,h:32},3)],
                                looping: false}).start())
                        }
                    }

                }
                state.level=2;    
                state.mode=Mode::Lvl2;           
            }
            (EntityType::Player, EntityType::lvl2Exit)  => {  
                state.camera = Vec2i(0,0); 
                state.mode=Mode::EndGame;
            }
            _ => {}
        }       
    }

    let mut tile_contacts= engine2d::collision::gather_contacts_tilemap(&state.positions, &state.sizes,&levels[state.level].0);
    engine2d::collision::restitute(&mut state.positions, &mut state.sizes,&mut tile_contacts);
    println!("entered lvl1 collision");
    println!("{:?}",state.level);

    // Update game rules: What happens when the player touches things?  When enemies touch walls?  Etc.
    // Maybe scroll the camera or change level
}
