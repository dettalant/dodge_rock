/*-------------------------------
            game_state.rs

  ゲーム内システム進行についてを、GameStateの形でまとめる
  画面描画については別（view.rsを参照）
    
  * struct Player: プレイヤーキャラについて。また今度別の場所に移したい
  * struct Actor : 意識を持つようにして動くもの。また今度別の場所に移したい。
  
  * impl GameState: ゲーム内システム進行について
    * new(): よくある初期化
    * main_game_system_loop(): メインゲームループを扱う
    * player_move(): 自機移動についてのもろもろ
  
  * axis_move()   : アナログスティック操作変数を、画面描画に役立つ形に直す
  * key_move()    : 十字キー操作変数を、画面描画に役立つ形に直す
  * player_move_speed(): いつかゲーム内から速度調整するためのバッファ
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
        // 自機移動
        self.player_move(input);
        
        Ok(())
    }
    
    // 自機移動をまとめる関数
    fn player_move(&mut self, input: &InputState) {
        if input.axis_lx != 0 || input.axis_ly != 0 {
            self.actor.player.x += axis_move(input.axis_lx);
            self.actor.player.y += axis_move(input.axis_ly);
        }
    
        // 十字キーの縦軸処理　上下動を判定
        if input.move_up || input.move_down {
            // もし左右キーも押されてたら、速度を8割に下げる
            let tmp_n = if input.move_left || input.move_right {
                0.8_f32
            } else {
                1_f32
            };
            
            self.actor.player.y += key_move(
                input.move_up, 
                input.move_down
            ) * tmp_n;
        }
        
        // 十字キーの横軸処理 左右動を判定
        if input.move_left || input.move_right {
            // もし上下キーも押されてたら、速度を8割に下げる
            let tmp_n = if input.move_up || input.move_down {
                0.8_f32
            } else {
                1_f32
            };
            
            self.actor.player.x += key_move(
                input.move_left, 
                input.move_right
            ) * tmp_n;
        }
    }
}



/// 縦軸あるいは横軸において、相反する移動結果を足して合わせる
/// 変数例: devide_move = move_up; add_move = move_down; 
fn key_move(divide_move: bool, add_move: bool) -> f32 {
    let n_devide = divide_move as i8 as f32;
    let n_add = add_move as i8 as f32; 
    
    let out_n = (-n_devide + n_add) * player_move_speed();
    
    out_n
}

/// 32768上限で送られてくる変数を、扱いやすい形にまとめる
///
/// axisの数値とキー入力の数値を合わせる役目が主体
fn axis_move(axis_value: i16) -> f32 {
    /* 
      負を負で割ったら反転しちゃう。
      valueが元々マイナスの数値を含むので、割る数は正数で良い。
      
      速度倍率を上げやすい仕組みにしておく。
      後々 v / n / m のm部分をゲームシステムから制御できるようにすればオッケー。
    */
    let out_n = if axis_value.is_positive() {
        // 最大で 1 * nになる
        axis_value as f32 / 32768_f32 * player_move_speed()
    } else {
        axis_value as f32 / 32758_f32 * player_move_speed()
    };
    
    out_n
}

/// 自機移動速度を調整する関数
fn player_move_speed() -> f32 {
    /*
      現状は直書き。
      後々でゲーム内変数から変更できるようにしたい。
      （例：スピードアップアイテム）
    */
    6_f32
}
