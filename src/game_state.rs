/*-------------------------------
            game_state.rs

  ゲーム内システム進行についてを、GameStateの形でまとめる
  画面描画については別（view.rsを参照）

-------------------------------*/ 

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
    
    pub fn main_game_system_loop(&mut self, input: &InputState) {
        if input.v_move != 0 || input.h_move != 0 {
            // とりあえず適当に動かしてみる
            self.actor.player.y += input.v_move as f32;
            self.actor.player.x += input.h_move as f32;
        } 
    }
}
