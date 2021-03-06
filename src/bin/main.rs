extern crate ggez;

extern crate dodge_rock;

use std::env;
use std::path::PathBuf;

// 可読性のため、use宣言を二つに分ける
use ggez::{ ContextBuilder };
use ggez::event::run;

use dodge_rock::{ args, etc };
use dodge_rock::core_state::CoreState;
use dodge_rock::conf::GameConf;

fn ggez_init(conf: GameConf) {
    // TODO 今は直書きだけど、後々変更を施したい
    let mut cb = ContextBuilder::new("dodge_rock", "dettalant")
        .window_setup(ggez::conf::WindowSetup::default()
            .title("Dodge Rock Game")
        )
        .window_mode(ggez::conf::WindowMode::default()
            .dimensions(360, 480)
        );
    
    if let Ok(manifest_dir) = env::var("CARGO_MANIFEST_DIR") {
        let mut path = PathBuf::from(manifest_dir);
        cb = cb.add_resource_path(path);
    } else {
        cb = cb.add_resource_path("./");
    }
    
    let ctx = &mut cb.build().expect("ggez ContextBuilderのエラー");
    
    // 了承なく.configと.localにフォルダ作るのやめてよggezくん
    let _ = etc::unused_dir_remove(ctx);
    
    match CoreState::new(ctx, conf) {
        Err(e) => {
            println!("ゲーム起動に失敗");
            println!("Error: {}", e);
        }
        Ok(ref mut game) => {
            let result = run(ctx, game);
            if let Err(e) = result {
                println!("Error: ゲーム実行中に何かが起きたようだ - {}", e);
            } else {
                println!("ゲームを終了します");
            }
        }
    }
}

fn main() {
    // 起動引数に基づく起動モードを指定
    args::Args::new();

    // ゲームの設定読み込みと環境変数指定
    let conf = GameConf::new("game_config.toml").unwrap();
    
    // ggezの初期化開始
    ggez_init(conf);
}
