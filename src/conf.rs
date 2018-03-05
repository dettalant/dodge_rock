/*-------------------------------
            conf.rs

  config.tomlなどの設定ファイルから、
  他で扱うデータを運び出す
  
  * GameConf impl
    * new()           : tomlから読み込んだ内容のうち、重要なものを環境変数に登録
    * toml_serde()    : tomlからデータを読み込む

  * toml deserialize用のstruct
    * GameOption
    * Assets
    * Translate

-------------------------------*/
//use std;
use std::io::Result;
use std::path::Path;

use etc;
use toml;

#[derive(Debug, Deserialize)]
pub struct GameConf {
   pub game_option: GameOption,
   pub assets: Assets,
   pub translate: Translate,
}

#[derive(Debug, Deserialize)]
pub struct GameOption {
   pub window_size: String,
   pub constant_fps: u32,
}

#[derive(Debug, Deserialize)]
pub struct Assets {
   pub assets_dir: String,
}

#[derive(Debug, Deserialize)]
pub struct Translate {
   pub is_translate: bool,
   pub translate_data_dir: String,
}

impl GameConf {
    /// tomlからデータを読み込み、重要なものを環境変数に登録
    ///
    /// 環境変数の呼び出しコードは、`src/conf.rs`冒頭コメントを参照のこと
    ///
    /// `GameConf::new("game_option.toml");` というふうに使う
    pub fn new<'a>(path_str: &'a str) -> Result<Self> {
        let conf_path = etc::eazy_path_set(path_str);
        let game_conf = GameConf::toml_serde(&conf_path)?;

        Ok(game_conf)
    }
    
    /// 内部用。tomlファイルを読み込んで、解析する関数を呼び出す
    fn toml_serde<'a>(path: &'a Path) -> Result<Self> {
        // 効率化のためにVec<u8>で受け取るようにするか
        let tmp_vec = etc::File::read_to_vec(path)?;
        let out_data = toml::de::from_slice(&tmp_vec).expect("Toml deserialize 時のエラー");
        Ok(out_data)
    }
}
