/*-------------------------------
            conf.rs

  config.tomlなどの設定ファイルから、
  他で扱うデータを運び出す
  
  * GameConf impl
    * new()           : tomlから読み込んだ内容のうち、重要なものを環境変数に登録
    * toml_serde()    : tomlからデータを読み込む
    * set_to_env_var(): 重要データを環境変数として追加する

  * toml deserialize用のstruct
    * GameOption
    * Assets
    * Translate

  # 環境変数リスト
  
  |   呼び出しコード            |            内容          |
  | "GAME_ASSETS_DIR"         | ゲームアセットフォルダの名前 |
  | "GAME_TRANSLATE_DATA_DIR" | 翻訳データフォルダの名前    |
  
  # 備考
  "GAME_TRANSLATE_DATA_DIR"は、
  `game_config.toml`の`is_translate`が`true`の時のみ登録される。
  
  # TODO
  boolはstd::env::varに登録できないので、
  "GAME_TRANSLATE_DATA_DIR"が空か否かで判定すること。
  
-------------------------------*/
use std;
use std::io::Result;
use std::path::Path;

use etc;
use toml;

#[derive(Debug, Deserialize)]
pub struct GameConf {
    game_option: GameOption,
    assets: Assets,
    translate: Translate,
}

#[derive(Debug, Deserialize)]
pub struct GameOption {
    window_size: String,
}

#[derive(Debug, Deserialize)]
pub struct Assets {
    assets_dir: String,
}

#[derive(Debug, Deserialize)]
pub struct Translate {
    is_translate: bool,
    translate_data_dir: String,
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
        
        // 各種環境変数への登録
        game_conf.set_to_env_var();

        Ok(game_conf)
    }
    
    /// 内部用。tomlファイルを読み込んで、解析する関数を呼び出す
    fn toml_serde<'a>(path: &'a Path) -> Result<Self> {
        // 効率化のためにVec<u8>で受け取るようにするか
        let tmp_vec = etc::file_read_to_vec(path)?;
        let out_data = toml::de::from_slice(&tmp_vec).expect("Toml deserialize 時のエラー");
        Ok(out_data)
    }
    
    fn set_to_env_var(&self) {
        use std::env::set_var;
        
        // assetsフォルダの名前を登録
        let game_assets_dir = "GAME_ASSETS_DIR";
        set_var(game_assets_dir, 
                &self.assets.assets_dir);
        
        // `is_translate == true`の場合、翻訳データフォルダを登録
        if self.translate.is_translate {
            let game_translate_data_dir = "GAME_TRANSLATE_DATA_DIR";
            set_var(game_translate_data_dir,
                    &self.translate.translate_data_dir);
        }
    }
}
