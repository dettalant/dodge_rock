/*-------------------------------
            game_state.rs

  ゲーム内システム進行についてを、GameStateの形でまとめる
  画面描画については別（view.rsを参照）
    
  * struct Player: プレイヤーキャラについて。また今度別の場所に移したい
  * struct Actor : 意識を持つようにして動くもの。また今度別の場所に移したい。
  * struct System: ゲームシステムに影響を持つ変数はここに。
  
  * impl GameState: ゲーム内システム進行について
    * new(): よくある初期化
    * main_game_system_loop(): メインゲームループを扱う
    * player_move(): 自機移動についてのもろもろ
    * player_collision_check(): 自機が画面外に出ないようにする（また今度当たり判定も取る）
  
  * axis_move()   : アナログスティック操作変数を、画面描画に役立つ形に直す
  * key_move()    : 十字キー操作変数を、画面描画に役立つ形に直す
  * player_move_speed(): いつかゲーム内から速度調整するためのバッファ
-------------------------------*/ 
use std;
use std::thread;
use std::sync::{ Arc, Mutex };
use ggez::{Context, GameResult};

use assets;
use input_state::InputState;

// また今度別ファイルに移行させたい
// 今は簡易版として、とりあえず形だけ作る
#[derive(Clone, Debug, Default)]
pub struct Player {
    /// 左右初期値
    pub x: f32,
    /// 上下初期値
    pub y: f32,
    /// 自機画像横幅
    pub width: u32,
    /// 自機画像縦幅
    pub height: u32,
}

#[derive(Clone, Debug)]
pub struct Actor {
    pub player: Player,
}

impl Actor {
    pub fn new(assets: &assets::Assets,
               system: &System) -> Actor {
        let player = Player {
            x: (system.window_w - assets.player_ship.width()) as f32 / 2_f32,
            y: system.window_h as f32 * 0.7,
            width: assets.player_ship.width(),
            height: assets.player_ship.height(),
        };
        
        Actor {
            player: player, // 雑な配置 
        }
    }
}

#[derive(Clone, Debug)]
/// ゲームシステムと関係する変数をここに入れる
pub struct System {
    pub window_h: u32,
    pub window_w: u32,
    /// 現在選択されているBGM番号
    pub current_bgm: u8,
    /// BGMを鳴らすか否か
    pub is_bgm: bool,
    pub is_bgm_paused: bool,
}

impl System {
    fn new(ctx: &mut Context) -> System {
        System {
            window_w: ctx.conf.window_mode.width,
            window_h: ctx.conf.window_mode.height,
            current_bgm: 1,
            is_bgm: true,
            is_bgm_paused: false,
        }
    }
}

#[derive(Clone, Debug)]
pub struct GameState {
    pub actor: Actor,
    pub system: System,
}

impl GameState {
    pub fn new(ctx: &mut Context, assets: &assets::Assets) -> GameState {
        let system = System::new(ctx);
        let actor = Actor::new(assets, &system);
        GameState {
            actor: actor,
            system: system,
        }
    }
    
    /// メインのゲーム画面を管理するやつ 
    pub fn main_game_mode(&mut self, input: &InputState) -> GameResult<()>{
        // 自機移動
        self.player_move(input);
        self.player_collision_check();
        
        Ok(())
    }
    
    /// 自機移動をまとめる関数
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
    
    // 自機が画面外に出ないようにチェック
    fn player_collision_check(&mut self) {
        // 暫定として適当に配置
        
        // 横軸画面端計算
        if self.actor.player.x.is_sign_positive() {
            let tmp_n = (self.system.window_w - self.actor.player.width) as f32;
            if self.actor.player.x > tmp_n {
                self.actor.player.x = tmp_n;
            }
        } else {
            self.actor.player.x = 0_f32;
        }
        
        // 縦軸画面端計算
        if self.actor.player.y.is_sign_positive() {
            let tmp_n = (self.system.window_h - self.actor.player.height) as f32;
            if self.actor.player.y > tmp_n {
                self.actor.player.y = tmp_n;
            }
        } else {
            self.actor.player.y = 0_f32;
        }
    }
    
    #[allow(dead_code)]
    pub fn change_bgm(&mut self, bgm_number: u8) {
        self.stop_bgm();
        let one_millis = std::time::Duration::from_millis(1000);
        std::thread::sleep(one_millis);
        
        self.system.current_bgm = bgm_number;
        self.play_bgm();
    }
    
    #[allow(dead_code)]    
    pub fn play_bgm(&mut self) {
        self.system.is_bgm = true;
    }
    
    #[allow(dead_code)]    
    pub fn pause_bgm(&mut self) {
        self.system.is_bgm_paused = true;
        self.system.is_bgm = false;
    }
    
    #[allow(dead_code)]
    pub fn stop_bgm(&mut self) {
        self.system.is_bgm_paused = false;
        self.system.is_bgm = false;
    }
    
    //#[allow(dead_code)]
    ///// ゲーム場面に応じたBGMを再生する
    //pub fn bgm_tuner(&self, 
                    //assets: &assets::Assets) -> GameResult<()> {
        //{
            //// テスト領域
            ////println!("is_bgm: {}, is_bgm_paused: {},\
            ////current_bgm: {}",
                ////self.system.is_bgm,
                ////self.system.is_bgm_paused,
                ////self.system.current_bgm);
        //}
        
        //// tmp_m変数にassetsの変数を束縛して、多様なトラックを一つの手段でかける
        //// 要するに曲番号一覧ってこと。
        //let tmp_m = match self.system.current_bgm {
            //0 => &assets.audio.no_sound,
            //1 => &assets.audio.test_bgm,
            //_ => &assets.audio.no_sound,
        //};
        
        //// ここ、マクロを使ってもう少しまともな形にしたい
        //if self.system.is_bgm {
            //// 1だとtest_bgmを再生
            //if tmp_m.paused() {
                //tmp_m.resume();
            //} else if tmp_m.stopped() {
                //tmp_m.play()?;
            //}
        //} else if self.system.is_bgm_paused && !self.system.is_bgm {
            //if tmp_m.playing() {
                //tmp_m.pause();
            //}
        //} else if !self.system.is_bgm {
            //if !tmp_m.stopped() {
                //tmp_m.stop();
            //}
        //}
        
        //Ok(())
    //}
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
