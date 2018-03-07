/*-------------------------------
            core_state.rs

  ユーザー入力の受け取り方をここで決めて、
  InputStateの形でまとめる。
  
  * impl CoreState: ゲームのガワを包む皮
    * new(): よくある初期化のやつ
  
  * Eventhandler for CoreState: ggezのイベントハンドラ
    * update()        : 多分描画した後の更新関連
    * draw()          : 画面を描画する系
    * key_down_event(): キーボードを押下した際のもの
    * key_up_event()  : キーボードを離した際のもの
    * controller_button_down_event(): コントローラー版押下
    * controller_button_up_event()  : コントローラー版放上
    * controller_axis_event: アナログスティックの動きを検知
    * focus_event: ウィンドウがアクティブになっているかを検知
  
  * print_debug: 起動時に一度のみデバッグモード文章を表示する
-------------------------------*/ 
use std::env;
//use std::thread;

use ggez::{ timer };
use ggez::{ Context, GameResult};
use ggez::event::{ Axis, Button, EventHandler, Keycode, Mod };

use assets::Assets;
use audio::GameAudio;
use conf::GameConf;
use input_state::InputState;
use game_state::GameState;
use view;

/// ゲームに使用する変数を一つにまとめる
pub struct CoreState {
    /// ウィンドウがアクティブになっているか否か
    pub has_focus: bool,
    /// ゲームアセットをひとまとめ
    pub assets: Assets,
    /// ゲーム内でのBGM関連まとめ
    pub audio: GameAudio,
    /// ユーザー操作をinputとして受ける
    pub input: InputState,
    /// ゲーム内で使う変数まとめ
    pub game_state: GameState,
    /// game_config.tomlから取得する情報がここに
    pub game_conf: GameConf,
}

impl CoreState {
    pub fn new(ctx: &mut Context, conf: GameConf) -> GameResult<CoreState> {
        // コンパイル通すためのむっちゃんこ汚いやり方。また直す。
        let assets_map = Assets::new_map(&conf)?;
        let audio = GameAudio::new(ctx, assets_map)?;
        
        let assets = Assets::new(ctx, &conf)?;
        let game_state = GameState::new(ctx, &assets);
        
        // "-d"引数を付けて起動した際のデバッグモード
        if env::var("GAME_ACTIVATE_MODE").unwrap() == "DEBUG_MODE" {
            print_debug(ctx, &game_state, &conf);
        }
        
        Ok(CoreState {
            has_focus: false,
            assets: assets,
            audio: audio,
            input: InputState::new(),
            game_state: game_state,
            game_conf: conf,
        })
    }
}

impl EventHandler for CoreState {
    fn update(&mut self, ctx: &mut Context) -> GameResult<()> {
        // 固定フレームレートで更新されるようにする
        // FPS上限値はgame_config.tomlから取得
        while timer::check_update_time(
            ctx, 
            self.game_conf.game_option.constant_fps) {        
            // ウィンドウがアクティブな際のみ更新させる
            if self.has_focus {
                self.game_state.main_game_mode(&self.input)?;
                //self.game_state.bgm_tuner(&self.assets)?;
                //self.audio.bgm_tuner(&self.input);
            }
        }
        Ok(())
    }
    
    fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
        // ゲーム描画のおまとめ関数
        view::render_game(self, ctx)?;
        
        Ok(())
    }
    
    fn key_down_event(&mut self, 
                      _ctx: &mut Context, 
                      keycode: Keycode, 
                      keymod: Mod, 
                      _repeat: bool) {
        // 押されたキーの処遇はInputStateで判定
        self.input.key_press(keycode, keymod);
    }
    fn key_up_event(&mut self, 
                    _ctx: &mut Context,
                    keycode: Keycode, 
                    keymod: Mod, 
                    _repeat: bool) {
        self.input.key_release(keycode, keymod);
    }
    
    fn controller_button_down_event(&mut self, _ctx: &mut Context, btn: Button, instance_id: i32) {
        // このゲームはおひとりさま専用でありんす
        if instance_id == 0 {
            self.input.pad_press(btn);
        }
    }

    fn controller_button_up_event(&mut self, _ctx: &mut Context, btn: Button, instance_id: i32) {
        // このゲームはおひとりさま専用(ry
        if instance_id == 0 {
            self.input.pad_release(btn);
        }
    }

    fn controller_axis_event(&mut self,
                             _ctx: &mut Context,
                             axis: Axis,
                             value: i16,
                             instance_id: i32) {
        if instance_id == 0 {
            self.input.axis_controll(axis, value);
        }
    }


    fn focus_event(&mut self, _ctx: &mut Context, gained: bool) {
        if gained {
            self.has_focus = true;
        } else {
            self.has_focus = false;
        }
    }
}

/// デバッグモードの際に、たまに参照したくなるデータを表示する
fn print_debug(ctx: &mut Context, 
               game_state: &GameState,
               conf: &GameConf) {
    let translate_dir = if conf.translate.is_translate {
        &conf.translate.translate_data_dir
    } else {
        "is_translateフラグ未指定"
    };
    
    let debug_text = format!("    \
    でばっぐもーど
  Window Size            : {} x {}
  Vsync                  : {}
  Constant Frame rate    : {}
  GAME_ACTIVATE_MODE     : {}
  GAME_ASSETS_DIR        : {}
  GAME_TRANSLATE_DATA_DIR: {}
  
  struct game_state      : {:?}
",  
        ctx.conf.window_mode.width,
        ctx.conf.window_mode.height,
        ctx.conf.window_mode.vsync,
        conf.game_option.constant_fps,
        env::var("GAME_ACTIVATE_MODE").unwrap(),
        conf.assets.assets_dir,
        translate_dir,
        game_state);
    
    println!("{}", debug_text);
}
