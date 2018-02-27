#[macro_use]
extern crate serde_derive;
extern crate toml;

mod args;
mod assets;
mod conf;
mod etc;

fn main() {
    // 起動引数にもどつく起動モードを取得
    let _args = args::Args::new();

    // ゲームの設定読み込みと環境変数指定
    let _conf = conf::GameConf::new("game_config.toml");

    let _assets = assets::Assets::new().unwrap();
    println!("{:?}", assets);
    
    println!("{:?}", assets.get("test.toml"));
}
