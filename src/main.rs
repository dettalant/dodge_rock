#[macro_use]
extern crate serde_derive;
extern crate toml;

mod args;
mod assets;
mod conf;
mod etc;

fn main() {
    // ゲームの設定読み込みと環境変数指定
    let conf = conf::GameConf::new("game_config.toml");

    // debug modeならtrue、そうでないならfalse
    let is_debug = args::new();
    println!("is_debug: {}", is_debug);
    
    let assets = assets::Assets::new().unwrap();
    println!("{:?}", assets);
    
    println!("{:?}", assets.get("test.toml"));
    
}
