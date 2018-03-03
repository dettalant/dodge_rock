/*-------------------------------
            input_state.rs

  
  
  ゲーム内処理についてはgame_state.rsを、
  画面描画についてはview.rsを参照のこと。

-------------------------------*/

use ggez::event::{ Axis, Button, Keycode, Mod };

/// ユーザー操作に対して、どういう効果をもたせるか
/// 基本的に0で動作を止めて、それ以外で動かす。
///
/// Axis基準で考えて、32768を一単位として使うことにした。
/// max数字がそれで、画面描画に繋がる部分で少なくする。
///
/// 0がニュートラルで、上に行けば-にいって、下に行けば+に行く。
/// アナログスティックの動きと、方向キーの数値を合わせる。
pub struct InputState {
    /// 上下移動。-で上、+で下に移動。
    pub v_move: i16,
    /// 左右移動。-で左、+で右に移動。
    pub h_move: i16,
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
        self.user_key_controll(keycode, true);
    }
    
    /// キーが離されたなら、入力信号をオフ(0)に
    pub fn key_release(&mut self,
                    keycode: Keycode,
                    _keymod: Mod) {
        // キーが離されたら移動をやめる
        self.user_key_controll(keycode, false);
    }
    
    pub fn pad_press(&mut self, btn: Button) {
        // 1pコンの入力のみ取得しているよ
        self.user_pad_controll(btn, true);
    }
    
    pub fn pad_release(&mut self, btn: Button) {
        // 1pコンでのみ動作
        self.user_pad_controll(btn, false);
    }
    
    // アナログスティック入力を振り分ける
    pub fn axis_controll(&mut self, axis: Axis, value: i16) {
        // 現状はテスト用
        
        match axis {
            Axis::LeftX => self.h_move = value,
            Axis::LeftY => self.v_move = value,
            _ => (),
        }
        
    }
    
    /// 入力キーごとに判定を変えるのがココ。キーボード用。
    fn user_key_controll(&mut self,
                         keycode: Keycode,
                         pressed: bool) {
        /*
          キーごとに反応を返す
          変数を少なくしようとboolで無い方法使ったけど、
          ちょっと失策だったかも。
          
          キー入力がある時:
          下, 右 == 1
          上, 左 == -1
          
          キー入力がない時:
          上下左右 == 0
        */
        let mut value = 0_i16;
        if pressed {
            value = 32767_i16;
        }
        
        match keycode {
            Keycode::Up => self.v_move = -value,
            Keycode::Down => self.v_move = value,
            Keycode::Left => self.h_move = -value,
            Keycode::Right => self.h_move = value,
            _ => (), // Do nothing
        }
    }
    
    /// 入力ボタンごとに判定を変える。ゲームパッド用。
    fn user_pad_controll(&mut self,
                         btn: Button,
                         pressed: bool) {
        /* 
          とりあえずわかりやすいボタンに対するやつ
          ggezの仕様的に、xbox360コン準拠。
          
          Axis入力についてはどないすっかな。
          
          ボタン入力がある時:
          下, 右 == 1
          上, 左 == -1
        
          キー入力がない時:
          上下左右 == 0
        */
        let mut value = 0_i16;
        if pressed {
            value = 32767_i16;
        }
        
        match btn {
            Button::DPadUp => self.v_move = -value,
            Button::DPadDown => self.v_move = value,
            Button::DPadLeft => self.h_move = -value,
            Button::DPadRight => self.h_move = value,
            _ => (), // Do nothing
        }
    }
}
