/*-------------------------------
            etc.rs

  カテゴリ分けに困る、
  細々とした関数をひとまとめにする
  
  * unused_dir_remove(): ggezが自動生成するフォルダを削除
-------------------------------*/
extern crate ggez;
use std;
use std::path::{ Path, PathBuf };
use std::io::Result;

// for File Read
use std::fs::File;
use std::io::{BufReader, Read};

pub fn file_read_to_string<'a>(path: &'a Path) -> Result<String> {
    let mut f = BufReader::new(File::open(path)?);
    let mut out_s = String::new();
    
    f.read_to_string(&mut out_s)?;
    Ok(out_s)
}

pub fn file_read_to_vec<'a>(path: &'a Path) -> Result<Vec<u8>> {
    let mut f = BufReader::new(File::open(path)?);
    let mut out_v = Vec::new();
    
    f.read_to_end(&mut out_v)?;
    Ok(out_v)
}

#[allow(dead_code)]
/// ggezが自動生成するフォルダを先んじて消しておく
pub fn unused_dir_remove(ctx: &mut ggez::Context) -> ggez::GameResult<()> {
    let user_conf_dir_path = ctx.filesystem.get_user_config_dir();
    let user_data_dir_path = ctx.filesystem.get_user_data_dir();

    // だいじょうぶだろうけど、一応エラー処理しておく

    if user_conf_dir_path.is_dir() {
        std::fs::remove_dir(user_conf_dir_path)?;
    }

    if user_data_dir_path.is_dir() {
        std::fs::remove_dir(user_data_dir_path)?;
    }

    Ok(())
}
/// デバッグ用の楽ちんpath設定
pub fn eazy_path_set<'a>(file_pos: &'a str) -> PathBuf {
    // file_pos = "game_setting.toml"
    let mut path_base = PathBuf::new();
    
    if let Ok(manifest_dir) = std::env::var("CARGO_MANIFEST_DIR") {
        path_base.push(manifest_dir);
    }
    path_base.push(file_pos);
    
    path_base
}
