use std::path::Path;
use std::rc::Rc;

use winit::window::WindowBuilder;
use winit::dpi::LogicalSize;
use winit::event::VirtualKeyCode;
use winit_input_helper::WinitInputHelper;

use engine2d::types::*;
use engine2d::graphics::Screen;
use engine2d::tiles::*;
use engine2d::animation::*;

// use engine2d::collision::*;
// Imagine a Resources struct (we'll call it AssetDB or Assets in the future)
// which wraps all accesses to textures, sounds, animations, etc.
use engine2d::resources::*;
use engine2d::texture::Texture;

const WIDTH: usize = 16*16;
const HEIGHT: usize = 16*15;

#[derive(Clone,Copy,PartialEq,Eq,Debug)]
enum EntityType {
    Player,
    Enemy
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

impl Mode {
    // update consumes self and yields a new state (which might also just be self)
    fn update(self, game:&mut GameState, input:&Input) -> Self {
        match self {
            Mode::Title => {
                if input.key_held(VirtualKeyCode::P){
                    Mode::Play(PlayMode::Map)
                } else {
                    self
                }
            },
            Mode::Play(pm) => {
                // Option-based approach; PlayMode decides what to change into.
                // Could return a Transition enum instead
                // if let Some(pm) = pm.update(game, input) {
                //     Mode::Play(pm)
                // } else {
                //     Mode::EndGame
                //     }  
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
                levels[1].0.draw(screen);
            },
            Mode::Play(pm) => {
                levels[0].0.draw(screen);
            },
            Mode::EndGame => {
                levels[2].0.draw(screen);
            }
        }
    }
}
impl PlayMode {
    fn update(self, _game:&mut GameState, input:&Input)-> Option<Self> {// 
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
            .with_title("Anim2D")
            .with_inner_size(size)
            .with_min_inner_size(size)
            .with_resizable(false)
    };
    // Here's our resources...
    let mut rsrc = Resources::new();
    let tileset = Rc::new(Tileset::new(
        vec![
            Tile{solid:true}, Tile{solid:true},
            Tile{solid:true}, Tile{solid:true},
            Tile{solid:true}, Tile{solid:true},
            Tile{solid:true}, Tile{solid:true},
            Tile{solid:true}, Tile{solid:true},
            Tile{solid:true}, Tile{solid:true},
            Tile{solid:true}, Tile{solid:true},
            Tile{solid:true}, Tile{solid:true},
            Tile{solid:true}, Tile{solid:true},
            Tile{solid:true}, Tile{solid:true},
        ],
        &rsrc.load_texture(Path::new("content/cliff.png"))
    ));
    let tileset1 = Rc::new(Tileset::new(
        vec![
            Tile{solid:true},
            Tile{solid:true},
            Tile{solid:true},
            Tile{solid:true},
        ],
        &rsrc.load_texture(Path::new("content/water.png"))
    ));

    // Here's our game rules (the engine doesn't know about these)
    let levels:Vec<Level> = vec![
        (
            // The map
            Tilemap::new(
                Vec2i(0,0),
                // Map size
                (16, 16),
                &tileset,
                // Tile grid
                vec![
                    0, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 2,
                    3, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 5,
                    6, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 8,
                    9, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 11,
                    12, 13, 13, 13, 13, 13, 13, 13, 13, 13, 13, 13, 13, 13, 13, 14,
                    15, 16, 16, 16, 16, 16, 16, 16, 16, 16, 16, 16, 16, 16, 16, 17,
                    1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1,
                    1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 
                    1, 1, 1, 1, 17, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1,
                    1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 
                    1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1,
                    1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 
                    1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1,
                    1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 
                    1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1,
                    1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 
                ],     
                
            ),
            // Initial entities on level start
            vec![
                (EntityType::Player, 2, 13),
                (EntityType::Enemy, 10, 13)
            ]
        ),
        // The End
        (Tilemap::new(
            Vec2i(0,0),
            // Map size
            (16, 16),
            &tileset,
            // Tile grid
            vec![
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
                1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1,
                1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 
            ],     
            
        ),
        // Initial entities on level start
        vec![
            (EntityType::Player, 0, 0),
            (EntityType::Enemy, 20, 0)
        ]
    ),
    (Tilemap::new(
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
        ],     
        
    ),
    // Initial entities on level start
    vec![
        (EntityType::Player, 0, 0),
        (EntityType::Enemy, 20, 0)
    ]
)
    ];
    let player_tex = rsrc.load_texture(Path::new("content/reaper.png"));
    let player_anim = Rc::new(Animation::freeze(Rect{x:0,y:0,w:26,h:36}));
    let enemy_tex = Rc::clone(&player_tex);
    let enemy_anim = Rc::new(Animation::freeze(Rect{x:0,y:0,w:26,h:36}));
    // ... more

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
    levels[state.level].0.draw(screen);
    for ((pos,tex),anim) in state.positions.iter().zip(state.textures.iter()).zip(state.anim_state.iter()) {
        screen.bitblt(tex,anim.frame(),*pos);
    }
    state.mode.display(&state, screen,levels);
}

fn update_game(resources:&Resources, levels: &Vec<Level>, state: &mut GameState, input: &WinitInputHelper, frame: usize) {
    // Player control goes here
    if input.key_held(VirtualKeyCode::Right) {
        state.velocities[0].0 = 2;
    }
    if input.key_held(VirtualKeyCode::Left) {
        state.velocities[0].0 = -2;
    }
    if input.key_held(VirtualKeyCode::Up) {
        state.velocities[0].1 = -2;
    }
    if input.key_held(VirtualKeyCode::Down) {
        state.velocities[0].1 = 2;
    }
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
