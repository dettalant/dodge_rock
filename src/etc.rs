/*-------------------------------
            etc.rs

  カテゴリ分けに困る、
  細々とした関数をひとまとめにする
  
  * file_read_to_string: stringとしてファイルを読み取る
  * file_read_to_vec   : Vec<u8>としてファイルを読み取る
  * unused_dir_remove(): ggezが自動生成するフォルダを削除
  * eazy_path_set()    : cargo環境でも通常環境でも適応できるpathをセット
-------------------------------*/
extern crate ggez;
use std;
use std::path::{ Path, PathBuf };
use std::io::Result;

// for File Read
use std::fs::File;
use std::io::{ BufReader, Read };

#[allow(dead_code)]
/// ファイルをStringとして読み込む
pub fn file_read_to_string<'a>(path: &'a Path) -> Result<String> {
    let mut f = BufReader::new(File::open(path)?);
    let mut out_s = String::new();
    
    f.read_to_string(&mut out_s)?;
    Ok(out_s)
}

#[allow(dead_code)]
/// ファイルをVec<u8>として読み込む
pub fn file_read_to_vec<'a>(path: &'a Path) -> Result<Vec<u8>> {
    let mut f = BufReader::new(File::open(path)?);
    let mut out_v = Vec::new();
    
    f.read_to_end(&mut out_v)?;
    Ok(out_v)
}

#[allow(dead_code)]
/// ggezが自動生成したフォルダを消去する
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

/// 手間が少ない楽ちんpath設定
///
/// cargo環境なら、プロジェクトディレクトリのpathから始める
///
/// そうでないなら、バイナリがあるフォルダから始める
pub fn eazy_path_set<'a>(path_str: &'a str) -> PathBuf {
    // 入力変数例: `path_str = "game_setting.toml"`
    let mut path_base = PathBuf::new();
    
    if let Ok(manifest_dir) = std::env::var("CARGO_MANIFEST_DIR") {
        path_base.push(manifest_dir);
    }
    path_base.push(path_str);
    
    path_base
}
