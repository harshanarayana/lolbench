#[macro_use]
extern crate log;

extern crate common;
extern crate gfx;
extern crate math;
extern crate wad;

extern crate num;
extern crate sdl2;
extern crate time;

pub use game::{Game, GameConfig};
pub use level::Level;

pub mod camera;
pub mod ctrl;
pub mod game;
pub mod level;
pub mod lights;
pub mod player;
pub mod world;

pub const SHADER_ROOT: &'static str = concat!(env!("CARGO_MANIFEST_DIR"), "../shaders");
