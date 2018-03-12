/*-------------------------------
            assets.rs

  ゲームアセットを処理する部分
  主にassetsフォルダからの自動読み込み
  
  * struct Dialog
  
  * Assets impl
    * new()               : 親フォルダ名から一括でデータ出力。外部用。
    * show_map()          :
    * debug_new()         : デバッグに役立つ簡易版
    * set_assets_map()    : 再帰読み込みした結果物を出力するやつ
    * set_assets_dir()    : 再帰読み込みを始める親フォルダを指定
    * recursive_read_dir(): 再帰読み込みしたデータをHashmapに放り込む
-------------------------------*/

use std::{ self, env };
use std::path::{ PathBuf };
use std::collections::HashMap;
use std::io::Result;

use ggez::{ Context, GameResult };
use ggez::graphics::{ self, Color, Font, Image, Rect,};

use etc;
use conf::GameConf;

/// ダイアログボックス用の変数まとめ
pub struct Dialog {
    pub title_box: Rect,
    pub go_box: Rect,
    pub go_box_color: Color,
    pub black_color: Color,
    pub default_color: Color,
}

impl Dialog {
    fn new(ctx: &mut Context) -> Self {
        
        let (window_w, window_h) = (
            ctx.conf.window_mode.width as f32,
            ctx.conf.window_mode.height as f32,
        );
        
        // ゲームオーバーダイアログのサイズ設定
        // 非常にマジックナンバーじみてるので、
        // 次のプロジェクトだとどうにかしたい
        let (go_box_w, go_box_h) = (window_w * 0.75,
                                    window_h * 0.46); 
        
        // 画面中央に、画面の六割ほどのダイアログボックスを表示
        let go_box = graphics::Rect::new(
            (window_w - go_box_w) / 2.0,
            (window_h - go_box_h) / 2.0,
            go_box_w,
            go_box_h,
        );
        
        // タイトルダイアログのサイズ設定
        let (title_box_w, title_box_h) = (
            window_w * 0.9,
            window_h * 0.57,
        );
        
        let title_box = graphics::Rect::new(
            (window_w - title_box_w) / 2.0,
            (window_h - title_box_h) / 2.0,
            title_box_w,
            title_box_h,
        );
        
        let go_box_color = Color::from_rgba(255, 255, 255, 220);
        
        let black_color = Color::from_rgba(0, 0, 0, 255);
        
        Dialog {
            go_box: go_box,
            go_box_color: go_box_color,
            title_box: title_box,
            black_color: black_color,
            default_color: graphics::get_color(ctx),
        }
    }
}

pub struct Assets {
    assets_map: HashMap<String, PathBuf>,
    pub dialog: Dialog,
    pub player_ship: Image,
    pub enemy_block: Image, 
    pub pixel_font: Font,
    pub pixel_font_small: Font,
    pub pixel_font_big: Font,
}

impl Assets {
    /// Assets structを生成する（あとで消すかも）
    pub fn new<'a>(ctx: &mut Context, conf: &'a GameConf) -> GameResult<Self> {
        let a_map = Assets::set_assets_map(conf)?;
        let player_ship = Image::new(
            ctx, 
            a_map.get("player_ship_29x48.png").unwrap(),
        )?; 
        
        let enemy_block = Image::new(
            ctx, 
            a_map.get("enemy_block_32x32.png").unwrap(),
        )?; 
        
        let pixel_font = Font::new(
            ctx,
            a_map.get("JF-Dot-MPlus12.ttf").unwrap(),
            18,
        )?;

        let pixel_font_small = Font::new(
            ctx,
            a_map.get("JF-Dot-MPlus12.ttf").unwrap(),
            15,
        )?;

        let pixel_font_big = Font::new(
            ctx,
            a_map.get("JF-Dot-MPlus12.ttf").unwrap(),
            26,
        )?;

        Ok(Assets {
            assets_map: a_map,
            dialog: Dialog::new(ctx),
            player_ship: player_ship,
            enemy_block: enemy_block,
            pixel_font: pixel_font,
            pixel_font_small: pixel_font_small,
            pixel_font_big: pixel_font_big,
        })
    }
    
    // 他所で使いまわすために、assets mapを参照渡し
    pub fn show_map<'a>(&'a self) -> &'a HashMap<String, PathBuf> {
        &self.assets_map
    }
    
    /// Assets mapだけを生成する（読み取るだけ）
    pub fn new_map<'a>(conf: &'a GameConf) -> GameResult<HashMap<String, PathBuf>> {
        let a_map = Assets::set_assets_map(conf)?;
        
        Ok(a_map)
    }
    /// assetsフォルダを楽ちんに読み込む
    pub fn set_assets_map<'a>(conf: &'a GameConf) -> Result<HashMap<String, PathBuf>> {
        // assetsフォルダは環境変数で取ってきてるよ
        let assets_path = Assets::set_assets_dir(&conf.assets.assets_dir);
        
        // assetsフォルダがなかったらエラーで落とす
        if !assets_path.exists() {
            panic!("Error: 存在しないassetsフォルダが指定された");
        }
        
        // 翻訳データ追加を想定して、先んじてVec<PathBuf>にしておく
        let mut include_dirs = vec![assets_path];
        
        // 環境変数に翻訳データが登録されてたら、そのディレクトリを追加
        // 安易な処理なので、もしかしたらバグが起きるかも。注意ね。
        if conf.translate.is_translate {
            let tmp_tl_path = etc::easy_path_set(&conf.translate.translate_data_dir);
            
            if !tmp_tl_path.exists() {
                panic!("Error: 存在しない翻訳データフォルダが指定された");
            }
            
            include_dirs.push(tmp_tl_path);
        }
        
        let assets_map = Assets::recursive_read_dir(include_dirs)?;

        Ok(assets_map)
    }
    
    /// 内部用。assetsフォルダのpathを取得する。
    fn set_assets_dir<'a>(assets_dir: &'a str) -> PathBuf {
        let mut assets_path = PathBuf::new();
        
        // cargoで起動したら、cargoのディレクトリをpathbufに追加
        // 何も考えず先人を真似たけど、これだけでcargo外に対応できるやつだった。
        if let Ok(manifest_dir) = env::var("CARGO_MANIFEST_DIR") {
            assets_path.push(manifest_dir);
        }
        
        // conf.rsで追加した環境変数から、assetsディレクトリ名を指定
        // もし環境変数になければpanic!する
        assets_path.push(assets_dir);

        assets_path
    }
    
    /// assetsフォルダから、再帰的にファイルパスを調べる
    fn recursive_read_dir<'a>(include_dirs: Vec<PathBuf>) -> Result<HashMap<String, PathBuf>> {
        // @in : assetsフォルダのpath（検索開始位置） 
        // @out: assets内全ファイルの名前とpath
        
        // わかりやすさのため、ここでmutフラグ付きの変数束縛をする
        let mut dir_tmp = include_dirs;
        let mut base = HashMap::new();
        
        // プロジェクトフォルダの場所を取り出す
        let base_path = etc::easy_path_set("");
        
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
                                             .expect("file name -> String変換時のエラー");
                            /* d.path()からbase_pathを引いて、
                               assetsフォルダ以降のpathを出す */
                            /*
                              ここ、ggezくんが"/foo/bar.png"みたいな指定
                              じゃないと受け取らないせいでこんがらがってる。
                              
                              どうして"foo/bar.png"指定じゃだめなんですか。
                              仕方がないので、強引に"/foo/bar.png"形式に修正。
                            */
                            let mut item_path = PathBuf::from("/");
                            let tmp_path = d.path()
                                .strip_prefix(&base_path)
                                .expect("path切り詰め時のエラー")
                                .to_path_buf();
                            item_path.push(&tmp_path);
                            base.insert(
                                item_name, 
                                item_path,
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
