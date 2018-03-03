/*-------------------------------
            game_state.rs

  ゲーム内システム進行についてを、GameStateの形でまとめる
  画面描画については別（view.rsを参照）
    
  * struct Player: プレイヤーキャラについて。また今度別の場所に移したい
  * struct Actor : 意識を持つようにして動くもの。また今度別の場所に移したい。
  
  * impl GameState: ゲーム内システム進行について
    * new(): よくある初期化
    * main_game_system_loop(): メインゲームループを扱う

  * move_adjust(): ユーザー操作から届いた変数を、画面描画に役立つ形に直す
-------------------------------*/ 
use ggez::GameResult;
use input_state::InputState;

// また今度別ファイルに移行させたい
// 今は簡易版として、とりあえず形だけ作る
#[derive(Debug, Default)]
pub struct Player {
    /// 左右
    pub x: f32,
    /// 上下
    pub y: f32,
}

#[derive(Debug)]
pub struct Actor {
    pub player: Player,
}

impl Actor {
    pub fn new() -> Actor {
        Actor {
            player: Player { x: 200_f32, y: 200_f32 }, // 雑な配置 
        }
    }
}

#[derive(Debug)]
pub struct GameState {
    pub actor: Actor,
}

impl GameState {
    pub fn new() -> GameState {
        GameState {
            actor: Actor::new(),
        }
    }
    
    pub fn main_game_system_loop(&mut self, input: &InputState) -> GameResult<()>{
        if input.v_move != 0 || input.h_move != 0 {
            // 
            self.actor.player.y += move_adjust(input.v_move);
            self.actor.player.x += move_adjust(input.h_move);
        }
        
        Ok(())
    }
}

/// 32768上限で送られてくる変数を、扱いやすい形にまとめる
///
/// axisの数値とキー入力の数値を合わせる役目が主体
fn move_adjust(value: i16) -> f32 {
    let mut out_n = 0_f32;
    
    /* 
      負を負で割ったら反転しちゃう。
      valueが元々マイナスの数値を含むので、割る数は正数で良い。
      
      速度倍率を上げやすい仕組みにしておく。
      後々 v / n / m のm部分をゲームシステムから制御できるようにすればオッケー。
    */
    if value.is_positive() {
        // 最大で 1 * nになる
        out_n = value as f32 / 32768_f32 * 6_f32;
    } else if value.is_negative() {
        out_n = value as f32 / 32758_f32 * 6_f32;
    }
    
    out_n
}
