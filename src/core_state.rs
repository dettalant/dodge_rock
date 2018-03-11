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
use ggez::{ self, Context, GameResult };
use ggez::event::{ Axis, Button, EventHandler, Keycode, Mod };

//use range_checker::{ Range, RangeImpl };

use assets::Assets;
use etc;
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
    /// ユーザー操作をinputとして受ける
    pub input: InputState,
    /// ゲーム内で使う変数まとめ
    pub game_state: GameState,
    /// game_config.tomlから取得する情報がここに
    pub game_conf: GameConf,
}

impl CoreState {
    pub fn new(ctx: &mut Context, conf: GameConf) -> GameResult<CoreState> {
        let assets = Assets::new(ctx, &conf)?;
        let mut game_state = GameState::new(ctx, &assets);
        
        // "-d"引数を付けて起動した際のデバッグモード
        if env::var("GAME_ACTIVATE_MODE").unwrap() == "DEBUG_MODE" {
            print_debug(ctx, &game_state, &conf);
        }
        
        game_state.actor.add_e_block(
            etc::random_x(game_state.system.window_w), 
            -50.0,
        );
        
        Ok(CoreState {
            has_focus: false,
            assets: assets,
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
                // フレーム数を計測して、時間を割り出す
                measure_time(
                    &mut self.game_state,
                    self.game_conf.game_option.constant_fps,
                );
                
                // メインのゲーム画面
                self.game_state.main_game_mode(&mut self.input)?;
                
            }
            
            if env::var("GAME_ACTIVATE_MODE").unwrap() == "DEBUG_MODE" {
                debug_frames(ctx, &mut self.game_state);
            }
        }
        Ok(())
    }
    
    fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {

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

fn measure_time(game_state: &mut GameState, constant_fps: u32) {
    // 常にフレーム数を計測
    game_state.system.frames += 1;
    
    if (game_state.system.frames % constant_fps as usize) == 0 {
        game_state.system.seconds += 1;
    }
}

/// デバッグ用のフレーム表示。とりあえず標準出力に出す。
fn debug_frames(ctx: &mut Context, game_state: &mut GameState) {   
    if game_state.system.frames % 60 == 0 {
        println!("FPS: {}, Seconds: {}, EnemyLen: {}", 
            ggez::timer::get_fps(ctx),
            game_state.system.seconds,
            game_state.actor.e_block.len());
        println!("Player.x: {}, Player.y: {}, Player.w: {}, Player.h: {}",
            game_state.actor.player.x,
            game_state.actor.player.y,
            game_state.actor.player.width,
            game_state.actor.player.height);
        println!("Player.collision: {}", game_state.actor.player.collision);
    }
}
