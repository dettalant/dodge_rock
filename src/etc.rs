/*-------------------------------
            etc.rs

  カテゴリ分けに困る、
  細々とした関数をひとまとめにする

  * impl File
    * read_to_string   : stringとしてファイルを読み取る
    * read_to_vec      : Vec<u8>としてファイルを読み取る

  * unused_dir_remove(): ggezが自動生成するフォルダを削除
  * eazy_path_set()    : cargo環境でも通常環境でも適応できるpathをセット
-------------------------------*/
use std;
use std::path::{ Path, PathBuf };
use std::io::Result;

// for File Read
use std::io::{ BufReader, Read };

// for ggez
use ggez;

pub struct File;

impl File {
    #[allow(dead_code)]
    /// ファイルをStringとして読み込む
    pub fn read_to_string<'a>(path: &'a Path) -> Result<String> {
        let mut f = BufReader::new(std::fs::File::open(path)?);
        let mut out_s = String::new();
        
        f.read_to_string(&mut out_s)?;
        Ok(out_s)
    }

    #[allow(dead_code)]
    /// ファイルをVec<u8>として読み込む
    pub fn read_to_vec<'a>(path: &'a Path) -> Result<Vec<u8>> {
        let mut f = BufReader::new(std::fs::File::open(path)?);
        let mut out_v = Vec::new();
        
        f.read_to_end(&mut out_v)?;
        Ok(out_v)
    }
    
    /// ディレクトリ内ファイル数を判定して、空ディレクトリなら削除
    pub fn empty_dir_remove<'a>(path: &'a Path) -> Result<()> {
        /*
          for文を回してファイル数計測
          絶対もっと良い方法あると思う（白目）
        */
        let mut cnt = 0;
        for _ in std::fs::read_dir(path)? {
            cnt += 1;
        }
        
        // ディレクトリ内に何もなかったらcntが0になる
        if cnt == 0 {
            std::fs::remove_dir(path)?;
        }
        
        Ok(())
    }
}

#[allow(dead_code)]
/// ggezが自動生成したフォルダを消去する
pub fn unused_dir_remove(ctx: &mut ggez::Context) -> ggez::GameResult<()> {
    let user_conf_dir_path = ctx.filesystem.get_user_config_dir();
    let user_data_dir_path = ctx.filesystem.get_user_data_dir();

    /* 
      let _ = std::fs::remove_dir()で問題ないことは知ってるけど、
      捨てていてもエラーを出すのは気になるの！
    */
    
    if user_conf_dir_path.is_dir() {
        File::empty_dir_remove(user_conf_dir_path)?;
    }

    if user_data_dir_path.is_dir() {
        File::empty_dir_remove(user_data_dir_path)?;
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
