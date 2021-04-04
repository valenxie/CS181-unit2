use std::path::Path;
use std::rc::Rc;
use std::{thread, time};

use winit::window::WindowBuilder;
use winit::dpi::LogicalSize;
use winit::event::VirtualKeyCode;
use winit_input_helper::WinitInputHelper;

use engine2d::types::*;
use engine2d::graphics::Screen;
use engine2d::tiles::*;
use engine2d::animation::*;
use engine2d::collision::*;

use std::fs::File;
use std::io::BufReader;
use std::time::Duration;
use rodio::{Decoder, OutputStream, Sink};
use rodio::source::{SineWave, Source};

// use engine2d::collision::*;
// Imagine a Resources struct (we'll call it AssetDB or Assets in the future)
// which wraps all accesses to textures, sounds, animations, etc.
use engine2d::resources::*;
use engine2d::texture::Texture;

const WIDTH: usize = 16*20;
const HEIGHT: usize = 16*20;

#[derive(Clone,Copy,PartialEq,Eq,Debug)]
enum EntityType {
    Player,
    Enemy,
    Blocker, 
    HBlocker,
    Destination,
}

type Level = (Tilemap, Vec<(EntityType, i32, i32)>);
type Input = WinitInputHelper;

#[derive(Debug,Clone,Copy)]
enum Mode {
    Title,
    Play(PlayMode),
    EndGame
}
#[derive(Debug,Clone,Copy)]
enum PlayMode {
    Map,
    Battle,
    Menu,
}

// Crates for music: kira, rodio. There will be a set up at the beginning of your program.
impl Mode {
    // update consumes self and yields a new state (which might also just be self)
    fn update(self, game:&mut GameState, input:&Input) -> Self {
        match self {
            Mode::Title => {
                // if input.mouse()!=None {
                //     println!("{}, {}", input.mouse().unwrap().0, input.mouse().unwrap().1);
                // }
                if input.key_held(VirtualKeyCode::P) || 
                   input.key_held(VirtualKeyCode::Return) ||
                   (input.mouse()!=None && input.mouse().unwrap().0 > 16.0*15.0
                   && input.mouse().unwrap().0 < 16.0*28.0 && input.mouse().unwrap().1 > 16.0*12.0
                   && input.mouse().unwrap().1 < 16.0*15.0
                   && input.mouse_pressed(0))
                {
                    game.level=1;
                    Mode::Play(PlayMode::Map)
                } else {
                    self
                }
            },
            Mode::Play(pm) => {
                // All velocities
                if game.movable {
                    // Option-based approach; PlayMode decides what to change into.
                    // Could return a Transition enum instead
                    // if let Some(pm) = pm.update(game, input) {
                    //     Mode::Play(pm)
                    // } else {
                    //     Mode::EndGame
                    // }  
                    // Player control goes here
                    if input.key_held(VirtualKeyCode::Right) {
                        if game.positions[0].0 < 16*19 {
                            game.velocities[0].0 = 2;
                        } else {
                            game.velocities[0].0 = 0;
                        }
                    } else if input.key_held(VirtualKeyCode::Left) {
                        if game.positions[0].0 > 0 {
                            game.velocities[0].0 = -2;
                        } else {
                            game.velocities[0].0 = 0;
                        }
                    } else {
                        game.velocities[0].0 = 0;
                    }
                    if input.key_held(VirtualKeyCode::Up) {
                        if game.positions[0].1 > 0 {
                            game.velocities[0].1 = -2;
                        } else {
                            game.velocities[0].1 = 0;
                        }
        
                        if game.positions[0].1 <= game.camera.1+16*5 && 
                            game.camera.1 > 0 {
                            game.camera.1 -= 2;
                        }
                    } else if input.key_held(VirtualKeyCode::Down) {
                        if game.positions[0].1 < 28*16 {
                            game.velocities[0].1 = 2;
                        } else {
                            game.velocities[0].1 = 0;
                        }
                    
                        if game.positions[0].1 >= game.camera.1+16*5 && 
                            game.camera.1 < 10*16 {
                            game.camera.1 += 2;
                        }
                    } else {
                        game.velocities[0].1 = 0;
                    }
                } else { // on not movable
                    game.velocities[0] = Vec2i(0,0);
                }

                // Determine blocker velocity
                if game.positions[3].0 >= 16*16 || game.positions[3].0 <= 2*16 {
                    game.velocities[3].0 *= -1;
                }

                // Determine enemy velocity - chasing the character
                if square(game.positions[1].0-game.positions[0].0)+square(game.positions[1].1-game.positions[0].1) <= 8000 {
                    if game.positions[1].0 > game.positions[0].0 {
                        game.velocities[1].0 = -1;
                    } else {
                        game.velocities[1].0 = 1;
                    }
                    if game.positions[1].1 > game.positions[0].1 {
                        game.velocities[1].1 = -1;
                    } else {
                        game.velocities[1].1 = 1;
                    }
                } else {
                    game.velocities[1] = Vec2i(0,0);
                }

                // Update all positions
                for (posn, vel) in game.positions.iter_mut().zip(game.velocities.iter()) {
                    posn.0 += vel.0;
                    posn.1 += vel.1;
                }

                // Die and return to start if touches fire
                if game.positions[0].0 <= 1*16 || game.positions[0].0 >= 17*16+8 {
                    game.movable = false;
                    // Play the explosion sound 
                    let (_, temp_stream_handle) = OutputStream::try_default().unwrap();
                    let temp_file = BufReader::new(File::open("content/jack/sound/explosion.wav").unwrap());
                    let temp_source = Decoder::new(temp_file).unwrap().amplify(5.0).take_duration(Duration::from_secs_f32(2.0));
                    game.soundstream.1.play_raw(temp_source.convert_samples()).unwrap();

                    let ten_millis = time::Duration::from_millis(500);
                    thread::sleep(ten_millis);
                    game.positions[0].0 = 9*16;
                    game.positions[0].1 = 0;
                    game.positions[1] = Vec2i(10*16, 25*16); 
                    game.camera = Vec2i(0, 0);
                    game.movable = true;
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
                levels[0].0.draw(screen);
            },
            Mode::Play(pm) => {
                levels[1].0.draw(screen);
                for ((pos,tex),anim) in game.positions.iter().zip(game.textures.iter()).zip(game.anim_state.iter()) {
                    screen.bitblt(tex,anim.frame(),*pos);
                }
            },
            Mode::EndGame => {
                levels[2].0.draw(screen);
            }
        }
    }
}
impl PlayMode {
    fn update(self, _game:&mut GameState, input:&Input) -> Option<Self> {// 
        match self {
            PlayMode::Map => {
                if input.key_held(VirtualKeyCode::M){
                    Some(PlayMode::Menu)
                }
                else if input.key_held(VirtualKeyCode::B){
                    Some(PlayMode::Battle)
                }
                else if input.key_held(VirtualKeyCode::R){
                    None
                }
                else{
                    Some(self)
                }
            },
            PlayMode::Menu => {
                if input.key_held(VirtualKeyCode::X){
                    Some(PlayMode::Map)
                }
                else{
                    Some(self)
                }
            },
            PlayMode::Battle => {
                if input.key_held(VirtualKeyCode::X){
                    Some(PlayMode::Map)
                }
                else{
                    Some(self)
                }
            }
        }
    }
    fn display(&self, _game:&GameState) {
        match self {
            PlayMode::Map => {
                println!("Map: m to menu, b to battle, e to end game");
            }
            PlayMode::Menu => {
                println!("Menu: x to exit menu");
            },
            PlayMode::Battle => {
                println!("Battle: x to exit battle");
            }
        }
    }
}
struct GameState {
    // Every entity has a position, a size, a texture, and animation state.
    // Assume entity 0 is the player
    // Current level
    level:usize,
    types: Vec<EntityType>,
    positions: Vec<Vec2i>,
    velocities: Vec<Vec2i>,
    sizes:Vec<(usize,usize)>,
    textures:Vec<Rc<Texture>>,
    anim_state:Vec<AnimationState>,
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
            .with_title("Anim2D")
            .with_inner_size(size)
            .with_min_inner_size(size)
            .with_resizable(false)
    };

    // Here's our resources...
    let mut rsrc = Resources::new();
    let tileset = Rc::new(Tileset::new(
        vec![
            Tile{solid:false}, Tile{solid:false}, Tile{solid:false}, Tile{solid:false},           
            Tile{solid:false}, Tile{solid:false}, Tile{solid:false}, Tile{solid:false},
            Tile{solid:false}, Tile{solid:false}, Tile{solid:false}, Tile{solid:false},
            Tile{solid:false}, Tile{solid:false}, Tile{solid:false}, Tile{solid:false},
            Tile{solid:false}, Tile{solid:false}, Tile{solid:false}, Tile{solid:false},
            Tile{solid:false}, Tile{solid:false}, Tile{solid:false}, Tile{solid:false},
            Tile{solid:false}, Tile{solid:false}, Tile{solid:false}, Tile{solid:false},
            Tile{solid:false}, Tile{solid:false}, Tile{solid:false}, Tile{solid:false},
            Tile{solid:false}, Tile{solid:false}, Tile{solid:false}, Tile{solid:false},           
            Tile{solid:false}, Tile{solid:false}, Tile{solid:false}, Tile{solid:false},
            Tile{solid:false}, Tile{solid:false}, Tile{solid:false}, Tile{solid:false},
            Tile{solid:false}, Tile{solid:false}, Tile{solid:false}, Tile{solid:false},
            Tile{solid:false}, Tile{solid:false}, Tile{solid:false}, Tile{solid:false},
            Tile{solid:false}, Tile{solid:false}, Tile{solid:false}, Tile{solid:false},
            Tile{solid:false}, Tile{solid:false}, Tile{solid:false}, Tile{solid:false},
            Tile{solid:false}, Tile{solid:false}, Tile{solid:false}, Tile{solid:false},
            Tile{solid:false}, Tile{solid:false}, Tile{solid:false}, Tile{solid:false},
            Tile{solid:false}, Tile{solid:false}, Tile{solid:false}, Tile{solid:false},
            Tile{solid:false}, Tile{solid:false}, Tile{solid:false}, Tile{solid:false},
            Tile{solid:false}, Tile{solid:false}, Tile{solid:false}, Tile{solid:false},
        ],
        &rsrc.load_texture(Path::new("content/jack/collage.png"))
    ));
    let tileset1 = Rc::new(Tileset::new(
        vec![
            Tile{solid:false},
            Tile{solid:false},
            Tile{solid:false},
            Tile{solid:false},
        ],
        &rsrc.load_texture(Path::new("content/jack/collage.png"))
    ));

    let tileset_title = Rc::new(Tileset::new(
        vec![ // Exactly 104 tiles. Which is 208 x 128 / (16 x 16)
            Tile{solid:false}, Tile{solid:false}, Tile{solid:false}, Tile{solid:false},           
            Tile{solid:false}, Tile{solid:false}, Tile{solid:false}, Tile{solid:false},
            Tile{solid:false}, Tile{solid:false}, Tile{solid:false}, Tile{solid:false},
            Tile{solid:false}, Tile{solid:false}, Tile{solid:false}, Tile{solid:false},
            Tile{solid:false}, Tile{solid:false}, Tile{solid:false}, Tile{solid:false},
            Tile{solid:false}, Tile{solid:false}, Tile{solid:false}, Tile{solid:false},
            Tile{solid:false}, Tile{solid:false}, Tile{solid:false}, Tile{solid:false},
            Tile{solid:false}, Tile{solid:false}, Tile{solid:false}, Tile{solid:false},
            Tile{solid:false}, Tile{solid:false}, Tile{solid:false}, Tile{solid:false},           
            Tile{solid:false}, Tile{solid:false}, Tile{solid:false}, Tile{solid:false},
            Tile{solid:false}, Tile{solid:false}, Tile{solid:false}, Tile{solid:false},
            Tile{solid:false}, Tile{solid:false}, Tile{solid:false}, Tile{solid:false},
            Tile{solid:false}, Tile{solid:false}, Tile{solid:false}, Tile{solid:false},
            Tile{solid:false}, Tile{solid:false}, Tile{solid:false}, Tile{solid:false},
            Tile{solid:false}, Tile{solid:false}, Tile{solid:false}, Tile{solid:false},
            Tile{solid:false}, Tile{solid:false}, Tile{solid:false}, Tile{solid:false},
            Tile{solid:false}, Tile{solid:false}, Tile{solid:false}, Tile{solid:false},
            Tile{solid:false}, Tile{solid:false}, Tile{solid:false}, Tile{solid:false},
            Tile{solid:false}, Tile{solid:false}, Tile{solid:false}, Tile{solid:false},
            Tile{solid:false}, Tile{solid:false}, Tile{solid:false}, Tile{solid:false},
            Tile{solid:false}, Tile{solid:false}, Tile{solid:false}, Tile{solid:false},
            Tile{solid:false}, Tile{solid:false}, Tile{solid:false}, Tile{solid:false},
            Tile{solid:false}, Tile{solid:false}, Tile{solid:false}, Tile{solid:false},
            Tile{solid:false}, Tile{solid:false}, Tile{solid:false}, Tile{solid:false},
            Tile{solid:false}, Tile{solid:false}, Tile{solid:false}, Tile{solid:false},
            Tile{solid:false}, Tile{solid:false}, Tile{solid:false}, Tile{solid:false},
            Tile{solid:false}, Tile{solid:false}, Tile{solid:false}, Tile{solid:false},
            Tile{solid:false}, Tile{solid:false}, Tile{solid:false}, Tile{solid:false},
            Tile{solid:false}, Tile{solid:false}, Tile{solid:false}, Tile{solid:false},
            Tile{solid:false}, Tile{solid:false}, Tile{solid:false}, Tile{solid:false},
            Tile{solid:false}, Tile{solid:false}, Tile{solid:false}, Tile{solid:false},
            Tile{solid:false}, Tile{solid:false}, Tile{solid:false}, Tile{solid:false},
            Tile{solid:false}, Tile{solid:false}, Tile{solid:false}, Tile{solid:false},
            Tile{solid:false}, Tile{solid:false}, Tile{solid:false}, Tile{solid:false},
            Tile{solid:false}, Tile{solid:false}, Tile{solid:false}, Tile{solid:false},
            Tile{solid:false}, Tile{solid:false}, Tile{solid:false}, Tile{solid:false},
            Tile{solid:false}, Tile{solid:false}, Tile{solid:false}, Tile{solid:false},
            Tile{solid:false}, Tile{solid:false}, Tile{solid:false}, Tile{solid:false},
            Tile{solid:false}, Tile{solid:false}, Tile{solid:false}, Tile{solid:false},
            Tile{solid:false}, Tile{solid:false}, Tile{solid:false}, Tile{solid:false},
            Tile{solid:false}, Tile{solid:false}, Tile{solid:false}, Tile{solid:false},
            Tile{solid:false}, Tile{solid:false}, Tile{solid:false}, Tile{solid:false},
            Tile{solid:false}, Tile{solid:false}, Tile{solid:false}, Tile{solid:false},
            Tile{solid:false}, Tile{solid:false}, Tile{solid:false}, Tile{solid:false},
            Tile{solid:false}, Tile{solid:false}, Tile{solid:false}, Tile{solid:false},
            Tile{solid:false}, Tile{solid:false}, Tile{solid:false}, Tile{solid:false},
            Tile{solid:false}, Tile{solid:false}, Tile{solid:false}, Tile{solid:false},
            Tile{solid:false}, Tile{solid:false}, Tile{solid:false}, Tile{solid:false},
            Tile{solid:false}, Tile{solid:false}, Tile{solid:false}, Tile{solid:false},
            Tile{solid:false}, Tile{solid:false}, Tile{solid:false}, Tile{solid:false},
            Tile{solid:false}, Tile{solid:false}, Tile{solid:false}, Tile{solid:false},
            Tile{solid:false}, Tile{solid:false}, Tile{solid:false}, Tile{solid:false},
            Tile{solid:false}, Tile{solid:false}, Tile{solid:false}, Tile{solid:false},
            Tile{solid:false}, Tile{solid:false}, Tile{solid:false}, Tile{solid:false},
            Tile{solid:false}, Tile{solid:false}, Tile{solid:false}, Tile{solid:false},
            Tile{solid:false}, Tile{solid:false}, Tile{solid:false}, Tile{solid:false},
            Tile{solid:false}, Tile{solid:false}, Tile{solid:false}, Tile{solid:false},
            Tile{solid:false}, Tile{solid:false}, Tile{solid:false}, Tile{solid:false},
            Tile{solid:false}, Tile{solid:false}, Tile{solid:false}, Tile{solid:false},
            Tile{solid:false}, Tile{solid:false}, Tile{solid:false}, Tile{solid:false},
            Tile{solid:false}, Tile{solid:false}, Tile{solid:false}, Tile{solid:false},
            Tile{solid:false}, Tile{solid:false}, Tile{solid:false}, Tile{solid:false},
            Tile{solid:false}, Tile{solid:false}, Tile{solid:false}, Tile{solid:false},
            Tile{solid:false}, Tile{solid:false}, Tile{solid:false}, Tile{solid:false},
            Tile{solid:false}, Tile{solid:false}, Tile{solid:false}, Tile{solid:false},
            Tile{solid:false}, Tile{solid:false}, Tile{solid:false}, Tile{solid:false},
            Tile{solid:false}, Tile{solid:false}, Tile{solid:false}, Tile{solid:false},
            Tile{solid:false}, Tile{solid:false}, Tile{solid:false}, Tile{solid:false},
            Tile{solid:false}, Tile{solid:false}, Tile{solid:false}, Tile{solid:false},
            Tile{solid:false}, Tile{solid:false}, Tile{solid:false}, Tile{solid:false},
            Tile{solid:false}, Tile{solid:false}, Tile{solid:false}, Tile{solid:false},
        ],
        &rsrc.load_texture(Path::new("content/jack/ahaha.png"))
    ));

    // Here's our game rules (the engine doesn't know about these)
    let levels:Vec<Level> = vec![
        // Title Screen
        (Tilemap::new(
            Vec2i(0,0),
            // Map size
            (20, 20),
            &tileset_title,
            // Tile grid
            vec![
                1,  1,  1,  1,  1,  1,  1,  1,  1,  1,  1,  1,  1,  1,  1,  1,  1,  1,  1,  1,
                1,  1,  1,  1,  1,  1,  1,  1,  1,  1,  1,  1,  1,  1,  1,  1,  1,  1,  1,  1,
                1,  1,  1,  1,  1,  1,  1,  1,  1,  1,  1,  1,  1,  1,  1,  1,  1,  1,  1,  1,
                1,  1,  1,  1,  1,  1,  1,  1,  1,  1,  1,  1,  1,  1,  1,  1,  1,  1,  1,  1,
                1,  1,  1,  1,  1,  1,  1,  1,  1,  1,  1,  1,  1,  1,  1,  1,  1,  1,  1,  1,
                1,  1,  1,  1,  1,  1,  1,  1,  1,  1,  1,  1,  1,  1,  1,  1,  1,  1,  1,  1,
                1,  1,  1,  1,  1,  1,  1,  1,  212,178,156,230,1,  1,  1,  1,  1,  1,  1,  1,
                1,  1,  1,  1,  1,  1,  1,  1,  1,  1,  1,  1,  1,  1,  1,  1,  1,  1,  1,  1,
                1,  1,  1,  1,  1,  1,  1,  1,  1,  1,  1,  1,  1,  1,  1,  1,  1,  1,  1,  1,
                1,  1,  1,  1,  1,  1,  1,  1,  1,  1,  1,  1,  1,  1,  1,  1,  1,  1,  1,  1,
                1,  1,  1,  1,  1,  1,  1,  1,  1,  1,  1,  1,  1,  1,  1,  1,  1,  1,  1,  1,
                1,  1,  1,  1,  1,  1,  1,  1,  1,  1,  1,  1,  1,  1,  1,  1,  1,  1,  1,  1,
                1,  1,  1,  1,  1,  1,  1,  1,  1,  1,  1,  1,  1,  1,  1,  1,  1,  1,  1,  1,
                1,  1,  1,  1,  1,  1,  1,  1,  1,  1,  1,  1,  1,  1,  1,  1,  1,  1,  1,  1,
                1,  1,  1,  1,  1,  1,  1,  1,  1,  1,  1,  1,  1,  1,  1,  1,  1,  1,  1,  1,
                1,  1,  1,  1,  1,  1,  1,  1,  1,  1,  1,  1,  1,  1,  1,  1,  1,  1,  1,  1,
                1,  1,  1,  1,  1,  1,  1,  1,  1,  1,  1,  1,  1,  1,  1,  1,  1,  1,  1,  1,
                1,  1,  1,  1,  1,  1,  1,  1,  1,  1,  1,  1,  1,  1,  1,  1,  1,  1,  1,  1,
                1,  1,  1,  1,  1,  1,  1,  1,  1,  1,  1,  1,  1,  1,  1,  1,  1,  1,  1,  1,
                1,  1,  1,  1,  1,  1,  1,  1,  1,  1,  1,  1,  1,  1,  1,  1,  1,  1,  1,  1,
            ],
        ),
        // Initial entities on level start
            vec![
                (EntityType::Player, 9, 0),
                (EntityType::Enemy, 20, 0)
            ]
        ),


        // First level - The map
        (Tilemap::new(
            Vec2i(0,0),
            // Map size
            (20, 30),
            &tileset,
            // Tile grid
            vec![
                3,  3,  1,  1,  1,  1,  1,  1,  1,  1,  1,  1,  1,  1,  1,  1,  1,  1,  3,  3,
                3,  3,  1,  1,  1,  1,  1,  1,  1,  1,  1,  1,  1,  1,  1,  1,  1,  1,  3,  3,
                3,  3,  20, 21, 21, 21, 21, 21, 21, 21, 21, 21, 22, 1,  1,  1,  1,  1,  3,  3,
                3,  3,  30, 31, 31, 31, 31, 31, 31, 31, 31, 31, 32, 1,  1,  1,  1,  1,  3,  3,
                3,  3,  30, 31, 31, 31, 31, 31, 31, 31, 31, 31, 32, 1,  1,  1,  1,  1,  3,  3,

                3,  3,  1,  1,  1,  1,  1,  1,  1,  1,  1,  1,  1,  1,  1,  1,  1,  1,  3,  3,
                3,  3,  1,  1,  1,  1,  1,  1,  1,  1,  1,  1,  1,  1,  1,  1,  1,  1,  3,  3,
                3,  3,  1,  1,  1,  1,  1,  1,  1,  1,  1,  1,  1,  1,  1,  1,  1,  1,  3,  3,
                3,  3,  1,  1,  1,  1,  1,  1,  1,  1,  1,  1,  1,  1,  1,  1,  1,  1,  3,  3,
                3,  3,  1,  1,  1,  1,  1,  1,  1,  1,  1,  1,  1,  1,  1,  1,  1,  1,  3,  3, 

                3,  3,  1,  1,  1,  1,  1,  1,  1,  1,  1,  1,  1,  1,  1,  1,  1,  1,  3,  3,
                3,  3,  1,  1,  1,  1,  1,  1,  1,  1,  1,  1,  1,  1,  1,  1,  1,  1,  3,  3,
                3,  3,  1,  1,  1,  1,  1,  1,  1,  1,  1,  1,  1,  1,  1,  1,  1,  1,  3,  3,
                3,  3,  1,  1,  1,  1,  1,  1,  1,  1,  1,  1,  1,  1,  1,  1,  1,  1,  3,  3,
                3,  3,  1,  1,  1,  1,  1,  1,  1,  1,  1,  1,  1,  1,  1,  1,  1,  1,  3,  3, 

                3,  3,  1,  1,  1,  1,  1,  1,  1,  1,  1,  1,  1,  1,  1,  1,  1,  1,  3,  3,
                3,  3,  1,  1,  1,  1,  1,  1,  1,  1,  1,  1,  1,  1,  1,  1,  1,  1,  3,  3,
                3,  3,  1,  1,  1,  1,  1,  1,  1,  1,  1,  1,  1,  1,  1,  1,  1,  1,  3,  3,
                3,  3,  1,  1,  1,  1,  1,  1,  1,  1,  1,  1,  1,  1,  1,  1,  1,  1,  3,  3,
                3,  3,  1,  1,  1,  1,  1,  1,  1,  1,  1,  1,  1,  1,  1,  1,  1,  1,  3,  3, 

                3,  3,  1,  1,  1,  1,  1,  1,  1,  1,  1,  1,  1,  1,  1,  1,  1,  1,  3,  3,
                3,  3,  1,  1,  1,  1,  1,  1,  1,  1,  1,  1,  1,  1,  1,  1,  1,  1,  3,  3,
                3,  3,  1,  1,  1,  1,  1,  1,  1,  1,  1,  1,  1,  1,  1,  1,  1,  1,  3,  3,
                3,  3,  1,  1,  1,  1,  1,  1,  1,  1,  1,  1,  1,  1,  1,  1,  1,  1,  3,  3,
                3,  3,  1,  1,  1,  1,  1,  1,  1,  1,  1,  1,  1,  1,  1,  1,  1,  1,  3,  3, 

                3,  3,  1,  1,  1,  1,  1,  1,  1,  1,  1,  1,  1,  1,  1,  1,  1,  1,  3,  3,
                3,  3,  1,  1,  1,  1,  1,  1,  1,  1,  1,  1,  1,  1,  1,  1,  1,  1,  3,  3,
                3,  3,  1,  1,  1,  1,  1,  1,  1,  1,  1,  1,  1,  1,  1,  1,  1,  1,  3,  3,
                3,  3,  1,  1,  1,  1,  1,  1,  1,  1,  1,  1,  1,  1,  1,  1,  1,  1,  3,  3,
                3,  3,  1,  1,  1,  1,  1,  1,  1,  1,  1,  1,  1,  1,  1,  1,  1,  1,  3,  3, 
                ],     
            ),
            // Plug in entities
            vec![
                (EntityType::Player, 10, 0),
                (EntityType::Enemy, 10, 25),
                (EntityType::Blocker, 5, 13), 
                (EntityType::HBlocker, 4, 3),
                (EntityType::Destination, 9, 27),
            ]
        ),

    (Tilemap::new(
        Vec2i(0,0),
        // Map size
        (20, 20),
        &tileset_title,
        // Tile grid
        vec![
            1,  1,  1,  1,  1,  1,  1,  1,  1,  1,  1,  1,  1,  1,  1,  1,  1,  1,  1,  1,
            1,  1,  1,  1,  1,  1,  1,  1,  1,  1,  1,  1,  1,  1,  1,  1,  1,  1,  1,  1,
            1,  1,  1,  1,  1,  1,  1,  1,  1,  1,  1,  1,  1,  1,  1,  1,  1,  1,  1,  1,
            1,  1,  1,  1,  1,  1,  1,  1,  1,  1,  1,  1,  1,  1,  1,  1,  1,  1,  1,  1,
            1,  1,  1,  1,  1,  1,  1,  1,  1,  1,  1,  1,  1,  1,  1,  1,  1,  1,  1,  1,
            1,  1,  1,  1,  1,  1,  1,  1,  1,  1,  1,  1,  1,  1,  1,  1,  1,  1,  1,  1,
            1,  1,  1,  1,  1,  1,  1,  1,  1,  1,  1,  1,  1,  1,  1,  1,  1,  1,  1,  1,
            1,  1,  1,  1,  1,  1,  1,  1,  1,  1,  1,  1,  1,  1,  1,  1,  1,  1,  1,  1,
            1,  1,  1,  1,  1,  1,  1,  1,  1,  1,  1,  1,  1,  1,  1,  1,  1,  1,  1,  1,
            1,  1,  1,  1,  1,  1,  1,  1,  1,  1,  1,  1,  1,  1,  1,  1,  1,  1,  1,  1,
            1,  1,  1,  1,  1,  1,  1,  1,  1,  1,  1,  1,  1,  1,  1,  1,  1,  1,  1,  1,
            1,  1,  1,  1,  1,  1,  1,  1,  1,  1,  1,  1,  1,  1,  1,  1,  1,  1,  1,  1,
            1,  1,  1,  1,  1,  1,  1,  1,  1,  1,  1,  1,  1,  1,  1,  1,  1,  1,  1,  1,
            1,  1,  1,  1,  1,  1,  1,  1,  1,  1,  1,  1,  1,  1,  1,  1,  1,  1,  1,  1,
            1,  1,  1,  1,  1,  1,  1,  1,  1,  1,  1,  1,  1,  1,  1,  1,  1,  1,  1,  1,
            1,  1,  1,  1,  1,  1,  1,  1,  1,  1,  1,  1,  1,  1,  1,  1,  1,  1,  1,  1,
            1,  1,  1,  1,  1,  1,  1,  1,  1,  1,  1,  1,  1,  1,  1,  1,  1,  1,  1,  1,
            1,  1,  1,  1,  1,  1,  1,  1,  1,  1,  1,  1,  1,  1,  1,  1,  1,  1,  1,  1,
            1,  1,  1,  1,  1,  1,  1,  1,  1,  1,  1,  1,  1,  1,  1,  1,  1,  1,  1,  1,
            1,  1,  1,  1,  1,  1,  1,  1,  1,  1,  1,  1,  1,  1,  1,  1,  1,  1,  1,  1,
        ],  
    ),
    // Initial entities on level start
    vec![
        (EntityType::Player, 0, 0),
        (EntityType::Enemy, 20, 0)
    ]
)
    ];
    // tex and anim
    let player_tex = rsrc.load_texture(Path::new("content/reaper.png"));
    let player_anim = Rc::new(Animation::freeze(Rect{x:5,y:5,w:25,h:35}));
    let enemy_tex = rsrc.load_texture(Path::new("content/jack/small_link.png"));
    let enemy_anim = Rc::new(Animation::freeze(Rect{x:0,y:0,w:26,h:36}));
    let blocker_tex = rsrc.load_texture(Path::new("content/jack/stone.png"));
    let blocker_anim = Rc::new(Animation::freeze(Rect{x:5,y:5,w:25,h:25}));
    let red_tex = rsrc.load_texture(Path::new("content/jack/red_rectangle.png"));
    let red_anim = Rc::new(Animation::freeze(Rect{x:0,y:0,w:32,h:32}));
    let dest_tex = rsrc.load_texture(Path::new("content/lvl1exit.png"));
    let dest_anim = Rc::new(Animation::freeze(Rect{x:0,y:0,w:64,h:48}));

    // game state definition
    let mut game = GameState{
        // Put an output stream here! *** 

        // Current level
        level: 0,
        // Every entity has a position, a size, a texture, and animation state.
        // Assume entity 0 is the player
        types: vec![
            // In a real example we'd provide nicer accessors than this
            levels[1].1[0].0,
            levels[1].1[1].0,
            levels[1].1[2].0,
            levels[1].1[3].0,
            levels[1].1[4].0,
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
            Vec2i(
                levels[1].1[4].1 * 16,
                levels[1].1[4].2 * 16,
            ),
        ],
        velocities: vec![Vec2i(0,0), Vec2i(0,0), Vec2i(0,0), Vec2i(1,0), Vec2i(0,0)],
        sizes: vec![(16,16), (16,16), (16,16), (32,32), (48,48)],
        // Could be texture handles instead, let's talk about that in two weeks
        textures: vec![Rc::clone(&player_tex),
                       Rc::clone(&enemy_tex), 
                       Rc::clone(&blocker_tex),
                       Rc::clone(&red_tex),
                       Rc::clone(&dest_tex),
                       ],
        anim_state: vec![player_anim.start(), enemy_anim.start(), blocker_anim.start(), red_anim.start(), dest_anim.start()],
        // Camera position
        camera: Vec2i(0, 0),
        mode:Mode::Title, 
        movable: true,
        soundstream: OutputStream::try_default().unwrap(),
    };

    // Music and Sound
    // Get a output stream handle to the default physical sound device
    // let (_stream, stream_handle) = OutputStream::try_default().unwrap();
    let file = BufReader::new(File::open("content/jack/sound/pockemon_center.mp3").unwrap());
    let source = Decoder::new(file).unwrap().delay(std::time::Duration::from_secs(5)).repeat_infinite();
    game.soundstream.1.play_raw(source.convert_samples()).unwrap();
    // let sink = Sink::try_new(&stream_handle).unwrap();
    // for i in 1..10 {
    //     // Load a sound from a file, using a path relative to Cargo.toml
    //     let file = BufReader::new(File::open("content/jack/sound/pockemon_center.mp3").unwrap());
    //     // Decode that sound file into a source
    //     let source = Decoder::new(file).unwrap().delay(std::time::Duration::from_secs(5));
    //     // Play the sound directly on the device
    //     sink.append(source);
    // }
    // sink.play();
    //stream_handle.play_raw(source.convert_samples());
    // The sound plays in a separate audio thread,
    // so we need to keep the main thread alive while it's playing.
    std::thread::sleep(std::time::Duration::from_secs(5));

    engine2d::run(WIDTH, HEIGHT, window_builder, rsrc, levels, game, draw_game, update_game);
}

fn draw_game(resources:&Resources, levels: &Vec<Level>, state: &GameState, screen: &mut Screen, frame:usize) {
    screen.clear(Rgba(80, 80, 80, 255));
    screen.set_scroll(state.camera);
    // levels[state.level].0.draw(screen);
    // for ((pos,tex),anim) in state.positions.iter().zip(state.textures.iter()).zip(state.anim_state.iter()) {
    //     screen.bitblt(tex,anim.frame(),*pos);
    // }
    state.mode.display(&state, screen, levels);
}

fn update_game(resources:&Resources, levels: &Vec<Level>, state: &mut GameState, input: &WinitInputHelper, frame: usize) {
    state.mode = state.mode.update(state, input);
    // Detect collisions: Convert positions and sizes to collision bodies, generate contacts
    let contacts = engine2d::collision::gather_contacts(&state.positions, &state.sizes);
    // Handle collisions: Apply restitution impulses.
    // Update game rules: What happens when the player touches things? When enemies touch walls? Etc.
    for contact in contacts.iter() {
        match (levels[state.level].1[contact.a].0, levels[state.level].1[contact.b].0) {
            (EntityType::Player, EntityType::Enemy) => {
                state.movable = false;
                // let (_, temp_stream_handle) = OutputStream::try_default().unwrap();
                let temp_file = BufReader::new(File::open("content/jack/sound/explosion.wav").unwrap());
                let temp_source = Decoder::new(temp_file).unwrap().amplify(5.0).take_duration(Duration::from_secs_f32(2.0));
                state.soundstream.1.play_raw(temp_source.convert_samples()).unwrap();
                let ten_millis = time::Duration::from_millis(500);
                // thread::sleep(ten_millis);
                state.positions[0].0 = 9*16;
                state.positions[0].1 = 0;
                state.positions[1] = Vec2i(10*16, 25*16); 
                state.camera = Vec2i(0, 0);
                state.movable = true;
            }
            (EntityType::Player, EntityType::Blocker) => {
                if state.velocities[0].0 != 0{
                    if state.positions[0].0 > state.positions[2].0 {
                        state.positions[0].0 += contact.mtv.0;
                    } else {
                        state.positions[0].0 -= contact.mtv.0;
                    }
                }
                if state.velocities[0].1 != 0 {
                    if state.positions[0].1 > state.positions[2].1 {
                        state.positions[0].1 += contact.mtv.1;
                    } else {
                        state.positions[0].1 -= contact.mtv.1;
                    }
                }
            }
            (EntityType::Player, EntityType::HBlocker) => {
                if state.positions[0].0 > state.positions[3].0 {
                    state.positions[0].0 += contact.mtv.0;
                } else {
                    state.positions[0].0 -= contact.mtv.0;
                }
            }
            (EntityType::Player, EntityType::Destination) => {
                state.camera = Vec2i(0,0); 
                state.mode = Mode::EndGame;
            }
            _ => {}
        }
    }

    // Maybe scroll the camera or change level
}
