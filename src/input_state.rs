/*-------------------------------
            input_state.rs
  
  ユーザー入力を処理する
  ゲーム内処理についてはgame_state.rsを、
  画面描画についてはview.rsを参照のこと。

  * struct InputState: キー入力が行われてるかのboolを貯めることにする
  
  * impl InputState: キー入力を適切な形で検知したい
    * new(): default()を呼ぶ初期化
    * reset(): キー入力のリセット
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
    /// 低速移動
    pub speed_down: bool,
    /// ゲームをリスタート
    pub game_reset: bool,
    /// ゲームオーバー画面からタイトル画面へ向かう
    pub game_title: bool,
    /// ゲームオーバー画面でゲームを終了させる
    pub game_quit: bool,
    /// なにかキーをおして〜〜用に、すべてのキーで反応するやつ
    pub any_key: bool,
    /// デバッグ用キー
    pub key_m: bool,
}

impl InputState {
    /// InputStateの初期化を行う
    pub fn new() -> InputState {
        let input: InputState = Default::default();
        input
    }
    
    /// キー入力をリセットする
    pub fn reset(&mut self) {
        // リセット用の変数束縛
        let input: InputState = Default::default();
        
        // 見よ、この強引なリセット！
        self.axis_lx = input.axis_lx;
        self.axis_ly = input.axis_ly;
        self.move_up = input.move_up;
        self.move_down = input.move_down;
        self.move_left = input.move_left;
        self.move_right = input.move_right;
        self.speed_down = input.speed_down;
        self.game_reset = input.game_reset;
        self.game_title = input.game_title;
        self.game_quit = input.game_quit;
        self.any_key = input.any_key;
        self.key_m = input.key_m;
    }
    
    /// キーが押されたら、入力信号をtrueに。キーボード用。
    pub fn key_press(&mut self, 
                  keycode: Keycode,
                  _keymod: Mod) {
        // キーが押されたら移動を始める
        self.user_key_controll(keycode, true);
    }
    
    /// キーが離されたなら、入力信号をfalseに。キーボード用。
    pub fn key_release(&mut self,
                    keycode: Keycode,
                    _keymod: Mod) {
        // キーが離されたら移動をやめる
        self.user_key_controll(keycode, false);
    }
    
    /// キーが押されたら、入力信号をtrueに。パッド用。
    pub fn pad_press(&mut self, btn: Button) {
        // 1pコンの入力のみ取得しているよ
        self.user_pad_controll(btn, true);
    }
    /// キーが離されたなら、入力信号をfalseに。パッド用。
    pub fn pad_release(&mut self, btn: Button) {
        // 1pコンでのみ動作
        self.user_pad_controll(btn, false);
    }
    
    // アナログスティック入力を振り分ける
    pub fn axis_controll(&mut self, axis: Axis, value: i16) {
        match axis {
            // 左スティックのみ使用
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
          
          キー入力がある時 == true
        
          キー入力がない時  == false
        */
        
        // 「なにかキーが押されたら〜〜」イベント用に
        self.any_key = pressed;
                
        // 方向キー、wasdキー、hjklキー(vim配列)に対応。
        match keycode {
            // 方向キー
            Keycode::Up     => self.move_up = pressed,
            Keycode::Down   => self.move_down = pressed,
            Keycode::Left   => self.move_left = pressed,
            Keycode::Right  => self.move_right = pressed,
            // wasdキー
            Keycode::W      => self.move_up = pressed,
            Keycode::A      => self.move_left = pressed,
            Keycode::S      => self.move_down = pressed,
            Keycode::D      => self.move_right = pressed,
            // vim配列(hjklキー)
            Keycode::K      => self.move_up = pressed,
            Keycode::H      => self.move_left = pressed,
            Keycode::J      => self.move_down = pressed,
            Keycode::L      => self.move_right = pressed,
            // 低速移動
            Keycode::LShift => self.speed_down = pressed,
            Keycode::RShift => self.speed_down = pressed,
            // ゲームリスタート
            Keycode::R      => self.game_reset = pressed,
            // タイトル画面へ
            Keycode::T      => self.game_title = pressed,
            // ゲーム終了
            Keycode::Q      => self.game_quit = pressed,
            // デバッグ用キー
            Keycode::M      => self.key_m = pressed,
            _ => (), // Do nothing
        }
    }
    
    /// 入力ボタンごとに判定を変える。ゲームパッド用。
    fn user_pad_controll(&mut self,
                         btn: Button,
                         pressed: bool) {
        /* 
          とりあえずわかりやすいボタンに対するやつ
          ggezの仕様でxbox360コン準拠。
          
          ボタン入力がある時 == true
        
          ボタン入力がない時 == false
        */
        
        self.any_key = pressed;
        
        match btn {
            // 十字キーでの移動
            Button::DPadUp => self.move_up = pressed,
            Button::DPadDown => self.move_down = pressed,
            Button::DPadLeft => self.move_left = pressed,
            Button::DPadRight => self.move_right = pressed,
            // 低速移動
            Button::LeftShoulder => self.speed_down = pressed,
            Button::RightShoulder => self.speed_down = pressed,
            _ => (), // Do nothing
        }
    }
}
