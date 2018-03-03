/*-------------------------------
            core_state.rs

  ユーザー入力の受け取り方をここで決めて、
  InputStateの形でまとめる。

-------------------------------*/ 

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
        Ok(CoreState {
            has_focus: false,
            assets: assets::Assets::new(ctx)?,
            input: InputState::new(),
            game_state: GameState::new(),
        })
    }
}

impl EventHandler for CoreState {
    fn update(&mut self, _ctx: &mut Context) -> GameResult<()> {
        if self.has_focus {
            self.game_state.main_game_system_loop(&self.input);
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


