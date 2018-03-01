/*-------------------------------
            input_state.rs

  
  
  ゲーム内処理についてはgame_state.rsを、
  画面描画についてはview.rsを参照のこと。

-------------------------------*/

use ggez::event::{ Keycode, Mod };

/// ユーザー操作に対して、どういう効果をもたせるか
/// 基本的に0で動作を止めて、それ以外で動かす。
pub struct InputState {
    /// 上下移動。1で上、-1で下に移動。
    pub v_move: i8,
    /// 左右移動。1で左、-1で右に移動。
    pub h_move: i8, 
}

/// InputStateの変数初期化。
impl Default for InputState {
    fn default() -> InputState {
        InputState {
            v_move: 0,
            h_move: 0,
        }
    }
}

impl InputState {
    /// InputStateの初期化を行う
    pub fn new() -> InputState {
        InputState::default()
    }
    
    /// キーが押されたら、入力信号をオン(1)に
    pub fn key_press(&mut self, 
                  keycode: Keycode,
                  _keymod: Mod) {
        // キーが押されたら移動を始める
        self.user_controll(keycode, 1);
    }
    
    /// キーが離されたなら、入力信号をオフ(0)に
    pub fn key_release(&mut self,
                    keycode: Keycode,
                    _keymod: Mod) {
        // キーが離されたら移動をやめる
        self.user_controll(keycode, 0);
    }
    
    /// 入力キーごとに判定を変えるのがココ
    fn user_controll(&mut self,
                     keycode: Keycode,
                     pressed: i8) {
        /*
          アナログ入力にも対応できるように、boolでない方法を使う
          
          キー入力がある時:
          上, 左 == 1
          下, 右 == -1
          
          キー入力がない時:
          上下左右 == 0
        */
        match keycode {
            Keycode::Up => self.v_move = pressed.abs(),
            Keycode::Down => self.v_move = -pressed.abs(),
            Keycode::Left => self.h_move = pressed.abs(),
            Keycode::Right => self.h_move = -pressed.abs(),
            _ => (), // Do nothing
        }
    }
}
