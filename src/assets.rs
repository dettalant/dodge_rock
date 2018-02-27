/*-------------------------------
            assets.rs

  ゲームアセットを処理する部分
  主にassetsフォルダからの自動読み込み
  
  * Assets impl
    * new()               : 親フォルダ名から一括でデータ出力。外部用。
    * set_assets_dir()    : 再帰読み込みを始める親フォルダを指定
    * recursive_read_dir(): 再帰読み込みしたデータをHashmapに放り込む
-------------------------------*/

use std::{ self, env };
use std::path::{ PathBuf };
use std::collections::HashMap;
use std::io::Result;

use etc;

pub struct Assets;

impl Assets {
    /// assetsフォルダを楽ちんに読み込む
    pub fn new<'a>() -> Result<HashMap<String, PathBuf>> {
        let assets_path = Assets::set_assets_dir();
        
        // assetsフォルダがなかったらエラーで落とす
        if !assets_path.exists() {
            panic!("Error: 存在しないassetsフォルダが指定された");
        }
        
        // 翻訳データ追加を想定して、先んじてVec<PathBuf>にしておく
        let mut include_dirs = vec![assets_path];
        
        // 環境変数に翻訳データが登録されてたら、そのディレクトリを追加
        // 安易な処理なので、もしかしたらバグが起きるかも。注意ね。
        if let Ok(translate_data_dir) = env::var("GAME_TRANSLATE_DATA_DIR") {
            let tmp_tl_path = etc::eazy_path_set(&translate_data_dir);
            
            if !tmp_tl_path.exists() {
                panic!("Error: 存在しない翻訳データフォルダが指定された");
            }
            
            include_dirs.push(tmp_tl_path);
        }
        
        let assets_map = Assets::recursive_read_dir(include_dirs)?;

        Ok(assets_map)
    }
    
    /// 内部用。assetsフォルダのpathを取得する。
    fn set_assets_dir<'a>() -> PathBuf {
        let mut assets_path = PathBuf::new();
        
        // cargoで起動したら、cargoのディレクトリをpathbufに追加
        // 何も考えず先人を真似たけど、これだけでcargo外に対応できるやつだった。
        if let Ok(manifest_dir) = env::var("CARGO_MANIFEST_DIR") {
            assets_path.push(manifest_dir);
        }
        
        // conf.rsで追加した環境変数から、assetsディレクトリ名を指定
        // もし環境変数になければpanic!する
        if let Ok(assets_dir) = env::var("GAME_ASSETS_DIR") {
            assets_path.push(assets_dir);
        } else {
            panic!("環境変数エラー: \"GAME_ASSETS_DIR\"が未指定");
        }
        assets_path
    }
    
    /// assetsフォルダから、再帰的にファイルパスを調べる
    fn recursive_read_dir<'a>(include_dirs: Vec<PathBuf>) -> Result<HashMap<String, PathBuf>> {
        // @in : assetsフォルダのpath（検索開始位置） 
        // @out: assets内全ファイルの名前とpath
        
        // わかりやすさのため、ここでmutフラグ付きの変数束縛をする
        let mut dir_tmp = include_dirs;
        let mut base = HashMap::new();
        
        // 今思うともう少しシンプルに書けたかも
        while dir_tmp.len() > 0 {
            let rdir = std::fs::read_dir(dir_tmp.swap_remove(0))?;
            
            for li in rdir {
                match li {
                    Ok(d) => {
                        if d.metadata()?.is_dir() {
                            dir_tmp.push(d.path());
                        } else if d.metadata()?.is_file() {
                            let item_name = d.file_name()
                                             .into_string()
                                             .unwrap();
                            base.insert(
                                item_name, 
                                d.path(),
                            );
                        } else {
                            // ファイルでもディレクトリでもない場合
                            // そうそうないだろうけど、一応実装
                            unreachable!("ファイルでもディレクトリでもない存在を検知")
                        }
                    }
                    Err(e) => println!("Asset読み込みエラー: {}", e),
                }
            }
        } // while end
        
        Ok(base)
    }
} 
