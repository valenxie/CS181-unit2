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
    Enemy
}

type Level = (Vec<Tilemap>, Vec<(EntityType, i32, i32)>);
type Input = WinitInputHelper;

#[derive(Debug,Clone,Copy)]
enum Mode {
    Title,
    Play,
    EndGame
}

impl Mode {
    // update consumes self and yields a new state (which might also just be self)
    fn update(self, game:&mut GameState, input:&Input) -> Self {
        match self {
            Mode::Title => {
                if input.key_held(VirtualKeyCode::P){
                    Mode::Play
                } else {
                    self
                }
            },
            Mode::Play => {
                // Option-based approach; PlayMode decides what to change into.
                // Could return a Transition enum instead
                // if let Some(pm) = pm.update(game, input) {
                //     Mode::Play(pm)
                // } else {
                //     Mode::EndGame
                //     } 
                if input.key_held(VirtualKeyCode::Right) {
                    if game.camera.0 < 256{
                        game.positions[0].0 += 2;
                        game.camera.0+=1;
                    }      
                }
                if input.key_held(VirtualKeyCode::Left) {
                    if game.camera.0 > -256{
                        game.positions[0].0 -= 2;
                        game.camera.0-=1;
                    }
                }
                if input.key_held(VirtualKeyCode::Up) {
                    if game.camera.1 > -256{
                        game.positions[0].1 -= 2;
                        game.camera.1-=1;
                    }   
                }
                if input.key_held(VirtualKeyCode::Down) {
                    if game.camera.1 < 0{
                        game.camera.1+=1; 
                        game.positions[0].1 += 2;
                    }
                } 
                if input.key_held(VirtualKeyCode::Q){
                    Mode::EndGame
                } else {
                    self
                }
            },
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
            Mode::Play=> {
                // screen.set_scroll(game.camera);
                for t in levels[1].0.iter(){
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
        mode:Mode
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
    let lvl1tileset = Rc::new(Tileset::new(
        vec![
            // (0..119).map(|_t| Tile{solid:false}).collect()
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
    let tileset1 = Rc::new(Tileset::new(
        vec![
            Tile{solid:false},
            Tile{solid:false},
            Tile{solid:false},
            Tile{solid:false},
        ],
        &rsrc.load_texture(Path::new("content/water.png"))
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
    let lvl1map1_1 = Tilemap::new(
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
    let lvl1map1_2 = Tilemap::new(
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
    let lvl1map1_3 = Tilemap::new(
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
    let lvl1map1_4 = Tilemap::new(
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
    let lvl1map1_5 = Tilemap::new(
        Vec2i(256, -256),
        (16, 16),
        &lvl1tileset,
        vec![
            16, 16, 16, 17, 130, 130, 130, 130, 130, 130, 130, 15, 40, 41, 42, 43, 
            21, 21, 21, 22, 130, 130, 130, 130, 130, 130, 130, 20, 45, 46, 47, 48,
            26, 26, 26, 27, 130, 130, 130, 130, 130, 130, 130, 25, 50, 51, 52, 53,
            1, 1, 1, 2, 130, 130, 130, 130, 130, 130, 130, 0, 1, 1, 1, 1,
            6, 30, 31, 7, 130, 130, 130, 130, 130, 130, 130, 5, 6, 6, 6, 6,
            6, 35, 36, 7, 130, 130, 130, 130, 130, 130, 130, 5, 6, 6, 6, 6,
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
    let lvl1map1_6 = Tilemap::new(
        Vec2i(-256, -256),
        (16, 16),
        &lvl1tileset,
        vec![
            130, 130, 130, 130, 130, 130, 130, 130, 130, 15, 16, 16, 16, 16, 16, 16,
            130, 130, 130, 130, 130, 130, 130, 130, 130, 20, 21, 21, 21, 21, 21, 21,
            130, 130, 130, 130, 130, 130, 130, 130, 130, 25,  26, 26, 26, 26, 26, 26,
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
    // Here's our game rules (the engine doesn't know about these)
    let levels:Vec<Level> = vec![
        (vec![start_screen],   
       // Initial entities on level start
         vec![
            (EntityType::Player, 8, 13),
            (EntityType::Enemy, 100, 100)
            ]
         ),

        (vec![lvl1map1_1,lvl1map1_2,lvl1map1_3,lvl1map1_4,lvl1map1_5,lvl1map1_6],
            // Initial entities on level start
         vec![
            (EntityType::Player, 8, 13),
            (EntityType::Enemy, 100, 100)
            ]
        ), 

        (vec![end_screen],   
         vec![
                (EntityType::Player, 0, 0),
                (EntityType::Enemy, 10, 0)
            ]
        ),   
    ];
    let player_tex = rsrc.load_texture(Path::new("content/player.png"));
    let player_anim = Rc::new(Animation::freeze(Rect{x:0,y:0,w:16,h:32}));
    let enemy_tex = Rc::clone(&player_tex);
    let enemy_anim = Rc::new(Animation::freeze(Rect{x:0,y:0,w:16,h:32}));

    // And here's our game state, which is just stuff that changes.
    // We'll say an entity is a type, a position, a velocity, a size, a texture, and an animation state.
    // State here will stitch them all together.
    let mut game = GameState{
        // Every entity has a position, a size, a texture, and animation state.
        // Assume entity 0 is the player
        types: vec![
            // In a real example we'd provide nicer accessors than this
            levels[0].1[0].0,
            levels[0].1[1].0,
        ],
        positions: vec![
            Vec2i(
                levels[0].1[0].1 * 16,
                levels[0].1[0].2 * 16,
            ),
            Vec2i(
                levels[0].1[1].1 * 16,
                levels[0].1[1].2 * 16,
            )
        ],
        velocities: vec![Vec2i(0,0), Vec2i(0,0)],
        sizes: vec![(16,16), (16,16)],
        // Could be texture handles instead, let's talk about that in two weeks
        textures: vec![Rc::clone(&player_tex),
                       Rc::clone(&enemy_tex)],
        anim_state: vec![player_anim.start(), enemy_anim.start()],
        // Current level
        level: 0,
        // Camera position
        camera: Vec2i(0, 0),
        mode:Mode::Title
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

    // Update all entities' positions
    for (posn, vel) in state.velocities.iter_mut().zip(state.positions.iter()) {
        posn.0 += vel.0;
        posn.1 += vel.1;
    }
    state.mode = state.mode.update(state, input);
    // Detect collisions: Convert positions and sizes to collision bodies, generate contacts

    // Handle collisions: Apply restitution impulses.

    // Update game rules: What happens when the player touches things?  When enemies touch walls?  Etc.

    // Maybe scroll the camera or change level
}
