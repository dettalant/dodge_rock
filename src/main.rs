#[macro_use]
extern crate serde_derive;
extern crate toml;

mod args;
mod assets;
mod conf;
mod etc;

fn main() {
    let conf_path = etc::eazy_path_set("game_config.toml");
    let conf = conf::GameConf::new(&conf_path);

    // debug modeならtrue、そうでないならfalse
    let is_debug = args::new();
    println!("is_debug: {}", is_debug);
    
    let assets = assets::Assets::new().unwrap();
    println!("{:?}", assets);
    
    println!("{:?}", assets.get("test.toml"));
    
}
