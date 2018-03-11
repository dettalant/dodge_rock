#[macro_use]
extern crate serde_derive;
extern crate toml;
extern crate ggez;
extern crate rand;

/// 自前のやつ
extern crate range_checker; 

pub mod args;
// audio機能は一旦凍結
//pub mod audio;
pub mod assets;
pub mod conf;
pub mod core_state;
pub mod etc;
pub mod game_state;
pub mod input_state;
pub mod view;
