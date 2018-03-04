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

// use ggez::{ graphics };
use ggez::{ Context, GameResult};
//use ggez::graphics::{ Point2 };
use ggez::event::{ Axis, Button, EventHandler, Keycode, Mod };

use assets;
use input_state::InputState;
use game_state::GameState;
use view;

/// ゲームに使用する変数を一つにまとめる
pub struct CoreState {
    /// ウィンドウがアクティブになっているか否か
    pub has_focus: bool,
    /// ゲームアセットをひとまとめ
    pub assets: assets::Assets,
    /// ユーザー操作をinputとして受ける
    pub input: InputState,
    /// ゲーム内で使う変数まとめ
    pub game_state: GameState,
}

impl CoreState {
    pub fn new(ctx: &mut Context) -> GameResult<CoreState> {
        let assets = assets::Assets::new(ctx)?;
        let game_state = GameState::new(ctx, &assets);
        
        // "-d"引数を付けて起動した際のデバッグモード
        if env::var("GAME_ACTIVATE_MODE").unwrap() == "DEBUG_MODE" {
            print_debug(ctx, &game_state);
        }
        
        Ok(CoreState {
            has_focus: false,
            assets: assets,
            input: InputState::new(),
            game_state: game_state,
        })
    }
}

impl EventHandler for CoreState {
    fn update(&mut self, _ctx: &mut Context) -> GameResult<()> {
        if self.has_focus {
            self.game_state.main_game_mode(&self.input)?;
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
               game_state: &GameState) {
    let translate_dir = match env::var("GAME_TRANSLATE_DATA_DIR") {
        Ok(s) => s,
        Err(_) => "未指定(translateフラグがオフ)".to_string(),
    };
    
    let debug_text = format!("    \
    でばっぐもーど
  Window Size            : {} x {}
  Vsync                  : {}
  GAME_ACTIVATE_MODE     : {}
  GAME_ASSETS_DIR        : {}
  GAME_TRANSLATE_DATA_DIR: {}
  
  struct game_state      : {:?}
",  
        ctx.conf.window_mode.width,
        ctx.conf.window_mode.height,
        ctx.conf.window_mode.vsync,
        env::var("GAME_ACTIVATE_MODE").unwrap(),
        env::var("GAME_ASSETS_DIR").unwrap(),
        translate_dir,
        game_state,);
    
    println!("{}", debug_text);
}
