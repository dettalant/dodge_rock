/*-------------------------------
            input_state.rs

  
  
  ゲーム内処理についてはgame_state.rsを、
  画面描画についてはview.rsを参照のこと。

  * struct InputState: キー入力が行われてるかのboolを貯めることにする
  
  * impl InputState: キー入力を適切な形で検知したい
    * default(): 初期化用のアレ
    * new(): default()を呼ぶ初期化
    * key_press()  : キーが押下されている場合の。バグがあるから修正したい。
    * key_release(): キーが放上された際の。バグがあるから修正したい。
    * pad_press()  : ゲームパッドボタンが押下された際の。なぜか上手く動く。
    * pad_release(): ゲームパッドボタンが放上された際の。なぜか動く。
    * axis_controll(): アナログスティックの管理。うまく動く。
    * user_key_controll(): 入力信号を変数へと変換する。キーボード用。
    * user_pad_controll(): 入力信号を変数へと変換する。ゲームパッド用。

-------------------------------*/

use ggez::event::{ Axis, Button, Keycode, Mod };

#[derive(Default)]
/// ユーザー操作に対して、どういう効果をもたせるか
/// 基本的に0で動作を止めて、それ以外で動かす。
///
/// Axis基準で考えて、32768を一単位として使うことにした。
/// max数字がそれで、画面描画に繋がる部分で少なくする。
///
/// 0がニュートラルで、上に行けば-にいって、下に行けば+に行く。
/// アナログスティックの動きと、方向キーの数値を合わせる。
pub struct InputState {
    /// 左アナログスティックの横軸
    pub axis_lx: i16,
    /// 左アナログスティックの縦軸
    pub axis_ly: i16,
    /// 十字キー上
    pub move_up: bool,
    /// 十字キー下
    pub move_down: bool,
    /// 十字キー左
    pub move_left: bool,
    /// 十字キー右
    pub move_right: bool,
}

impl InputState {
    /// InputStateの初期化を行う
    pub fn new() -> InputState {
        let input: InputState = Default::default();
        input
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
        
        match axis {
            Axis::LeftX => self.axis_lx = value,
            Axis::LeftY => self.axis_ly = value,
            _ => (),
        }
        
    }
    
    /// 入力キーごとに判定を変えるのがココ。キーボード用。
    fn user_key_controll(&mut self,
                         keycode: Keycode,
                         pressed: bool) {
        /*
          キーごとに反応を返す
          愚直にboolを使うことにした
          
          ボタン入力がある時:
          上下左右 == true
        
          キー入力がない時:
          上下左右 == false
        */
        
        match keycode {
            Keycode::Up => self.move_up = pressed,
            Keycode::Down => self.move_down = pressed,
            Keycode::Left => self.move_left = pressed,
            Keycode::Right => self.move_right = pressed,
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
          
          ボタン入力がある時:
          上下左右 == true
        
          キー入力がない時:
          上下左右 == false
        */
        
        match btn {
            Button::DPadUp => self.move_up = pressed,
            Button::DPadDown => self.move_down = pressed,
            Button::DPadLeft => self.move_left = pressed,
            Button::DPadRight => self.move_right = pressed,
            _ => (), // Do nothing
        }
    }
}
