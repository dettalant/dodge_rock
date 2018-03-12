/*-------------------------------
            game_state.rs

  ゲーム内システム進行についてを、GameStateの形でまとめる
  画面描画については別（view.rsを参照）
    
  * struct Player: プレイヤーキャラについて。
  * struct Enemy : 敵キャラ
  * struct Template: clone()元になるテンプレート入れ
  * struct Actor :  意識を持つようにして動くもの。また今度別の場所に移したい。
  * struct System: ゲームシステムに影響を持つ変数はここに。
  
  * impl Template:
    * new()
    
  * impl Actor: 
    * new()
    * reset()
    * add_e_block()

  * impl System:
    * new()
    * reset()
  
  * impl GameState: ゲーム内システム進行について
    * new(): よくある初期化
    * title_mode()
    * game_over_mode()
    * game_reset() : リスタート用に一部変数を初期化
    * main_game_system_loop(): メインゲームループを扱う
    * player_move(): 自機移動についてのもろもろ
    * player_move_speed(): 現在の自機移動速度を出す
    * player_collision_check(): 自機が画面外に出ないようにする（また今度当たり判定も取る）
    * enemy_move()
    * enemy_move_speed()
    * enemy_move_speed_adjust()
    * enemy_collision_check()
    * enemy_pop()
    * enemy_pop_width()
    * debug_key()
  
  * axis_move()   : アナログスティック操作変数を、画面描画に役立つ形に直す
  * key_move()    : 十字キー操作変数を、画面描画に役立つ形に直す
-------------------------------*/ 

use std::env;
use std::thread;

use ggez::{ Context, GameResult };
use range_checker::{ Range2D, Range2DImpl };

use assets;
use etc;
use input_state::InputState;

// また今度別ファイルに移行させたい
// 今は簡易版として、とりあえず形だけ作る
#[derive(Clone, Debug, Default)]
/// 自機が保有する変数セット
pub struct Player {
    /// 左右初期値
    pub x: f32,
    /// 上下初期値
    pub y: f32,
    /// 自機画像横幅
    pub width: u32,
    /// 自機画像縦幅
    pub height: u32,
    /// 当たり判定
    pub collision: Range2D<f32>
}

#[derive(Clone, Debug, Default)]
/// 敵一体一体が保有する変数セット
pub struct Enemy {
    /// 左右座標値
    pub x: f32,
    /// 上下座標値
    pub y: f32,
    /// 画像横幅
    pub width: u32,
    /// 画像縦幅
    pub height: u32,
    // 当たり判定
    pub collision: Range2D<f32>,
}

#[derive(Clone, Debug)]
/// 敵追加のためのテンプレート
pub struct Template {
    pub player: Player,
    pub e_block: Enemy,
}

impl Template {
    /// struct Templateを生成する
    pub fn new(assets: &assets::Assets, 
               system: &System) -> Self {
        let player = Player {
            x: (system.window_w - assets.player_ship.width()) as f32 / 2_f32,
            y: system.window_h as f32 * 0.7,
            width: assets.player_ship.width(),
            height: assets.player_ship.height(),
            collision: Range2D::default(),
        };
        
        let e_block = Enemy {
            x: 0.0,
            y: 0.0,
            width: assets.enemy_block.width(),
            height: assets.enemy_block.height(),
            collision: Range2D::default(),
        };
        
        Template {
            player: player,
            e_block: e_block,
        }
    }
}

#[derive(Clone, Debug)]
/// 自機や敵など、動いてゲームに関わるアクター
pub struct Actor {
    pub player: Player,
    pub e_block: Vec<Enemy>,
    template: Template 
}

impl Actor {
    /// struct Actorを生成する
    pub fn new(assets: &assets::Assets,
               system: &System) -> Self {        

        let template = Template::new(assets, system);
        let player = template.player.clone();
        
        Actor {
            player: player,
            e_block: Vec::<Enemy>::new(),
            template: template,    
        }
    }
    
    /// 敵を一体追加する
    pub fn add_e_block(&mut self, x: f32, y: f32) {
        let mut tmp_e = self.template.e_block.clone();
        tmp_e.x = x;
        tmp_e.y = y;
        
        self.e_block.push(tmp_e);
    }
    
    // ゲームシステムに関わる部分をリセット
    fn reset(&mut self) {
        self.player = self.template.player.clone();
        self.e_block = Vec::<Enemy>::new();
    }
}

#[derive(Clone, Debug)]
/// ゲームシステムと関係する変数をここに入れる
pub struct System {
    /// ウィンドウサイズ横幅
    pub window_w: u32,
    /// ウィンドウサイズ縦幅
    pub window_h: u32,
    /// 耐久フレーム数
    pub frames: usize,
    /// 耐久秒数
    pub seconds: usize,
    /// タイトル画面が表示されているか否か
    pub is_title: bool,
    /// ゲームオーバー画面が表示されているか否か
    pub is_game_over: bool,
    /// スコアが出力されたかどうか
    pub is_score_wrote: bool,
    /// 自機移動速度
    pub player_move_speed: f32,
    /// 敵移動速度
    pub enemy_move_speed: f32,
}

impl System {
    fn new(ctx: &mut Context) -> System {
        System {
            window_w: ctx.conf.window_mode.width,
            window_h: ctx.conf.window_mode.height,
            frames: 0,
            seconds: 0,
            is_title: true,
            is_game_over: false,
            is_score_wrote: false,
            player_move_speed: 2.0,
            enemy_move_speed: 1.0,
        }
    }
    
    // ゲームシステムに関わる部分をリセット
    fn reset(&mut self) {
        self.frames = 0;
        self.seconds = 0;
        self.enemy_move_speed = 1.0;
        self.is_score_wrote = false;
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
    
    /// タイトル画面を管理
    pub fn title_mode(&mut self,
                      input: &mut InputState) {
        if input.any_key {
            self.system.is_title = false;
        }
    }
    /// ゲームオーバー時の画面を管理
    pub fn game_over_mode(&mut self,
                          ctx: &mut Context,
                          input: &mut InputState) {
        if input.game_reset {
            // ゲームを再度はじめる
            self.game_reset();
            self.system.is_game_over = false;
        } else if input.game_title {
            // タイトル画面へ
            self.game_reset();
            self.system.is_game_over = false;
            self.system.is_title = true;
            input.reset();
        } else if input.game_quit {
            // ゲーム終了
            ctx.quit().expect("ゲーム終了時のエラー");
        }
    }
    
    /// ゲーム状態を初期化する
    pub fn game_reset(&mut self) {
        // struct System の初期化
        self.system.reset();
        
        // struct Actor の初期化
        self.actor.reset();

        // 敵キャラを一体出しておく
        let tmp_n = self.enemy_pop_width();
        self.actor.add_e_block(
            etc::random_x(tmp_n), 
            -50.0,
        );
        
        // タイトル画面を実装したら、タイトル画面は省く処理が必要
    }
    
    /// メインのゲーム画面を管理するやつ 
    pub fn main_game_mode(&mut self, input: &mut InputState) -> GameResult<()> {
        // 自機移動
        self.player_move(input);
        // 自機が画面外に出ないようにチェック
        self.player_collision_check();
        // 敵が下方向に移動
        self.enemy_move();
        // 敵を増殖
        self.enemy_pop();
        // 現状ではプレイヤーの当たり判定を見る
        self.enemy_collision_check();
        
        if env::var("GAME_ACTIVATE_MODE").unwrap() == "DEBUG_MODE" {
            self.debug_key(input);
        }
        
        Ok(())
    }
    
    /// 自機移動をまとめる関数
    fn player_move(&mut self, input: &InputState) {
        // アナログスティック処理のため、tmp変数にx,y値を入れる
        let (mut tmp_x, mut tmp_y) = if input.axis_lx != 0 || input.axis_ly != 0 {
            (axis_move(input.axis_lx), axis_move(input.axis_ly))
        } else {
            (0.0, 0.0)
        };
        
        // キー入力数を取得
        let move_key_cnt = input.move_up as i8 + input.move_down as i8 +
                           input.move_left as i8 + input.move_right as i8;
        
        // 二つ以上のキー入力があったら、移動速度を8割にする
        let tmp_speed_n = if move_key_cnt >= 2 {
            0.8
        } else {
            1.0
        };
        
        // 十字キーの縦軸処理　上下動を判定
        if input.move_up || input.move_down {
            tmp_y += key_move(
                input.move_up, 
                input.move_down
            ) * tmp_speed_n
        }
        
        // 十字キーの横軸処理 左右動を判定
        if input.move_left || input.move_right {
            
            tmp_x += key_move(
                input.move_left, 
                input.move_right
            ) * tmp_speed_n
        }
        
        // 移動値を足して完成
        self.actor.player.x += tmp_x * self.player_move_speed(input);
        self.actor.player.y += tmp_y * self.player_move_speed(input);
    }
    
    /// 自機移動速度を調整する関数
    fn player_move_speed(&mut self, input: &InputState) -> f32 {
        // 低速移動ボタンを押下中は速度半減
        let slow_buff = if input.speed_down {
            0.6
        } else {
            1.0
        };
        
        3.0 * self.system.player_move_speed * slow_buff
    }
    
    /// 自機が画面外に出ないようにチェック
    fn player_collision_check(&mut self) {
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
        
        // 自機当たり判定を代入
        self.actor.player.collision = Range2D::new(
            self.actor.player.x + 10.0,
            self.actor.player.y + 22.0,
            (self.actor.player.width - 20) as f32,
            (self.actor.player.height - 35) as f32,
        );
    }
    
    /// 敵の移動を自動で行い、コリジョン値を指定する
    fn enemy_move(&mut self) {
        self.enemy_move_speed_adjust();
        
        // e_blockたちの表示座標を動かす
        for i in 0..self.actor.e_block.len() {
            self.actor.e_block[i].y += 1.0 * self.enemy_move_speed();
            // 画面外の一定地点に移動したら、上へとループさせる
            if self.actor.e_block[i].y >= (self.system.window_h + 50) as f32 {
                self.actor.e_block[i].x = etc::random_x(
                    self.system.window_w - self.actor.template.e_block.width
                );
                self.actor.e_block[i].y = -50.0;
            }
            
            // e_blockのコリジョンを追加（雑なやり方）
            self.actor.e_block[i].collision = Range2D::new(
                self.actor.e_block[i].x,
                self.actor.e_block[i].y,
                self.actor.e_block[i].width as f32,
                self.actor.e_block[i].height as f32,
            );
        }// end for
    }
    
    /// 徐々に敵速度を上昇させていく関数
    fn enemy_move_speed_adjust(&mut self) {
        // 一定期間に一回判定するためのboolを作る
        // 一秒一回だと若干速度上昇が露骨なので、0.5秒に一回
        let is_speed_up = self.system.frames % 30 == 0; 
        
        // はじめは早く速度上昇させて、それ移行はゆっくりに
        if is_speed_up && self.system.enemy_move_speed < 5.0 {
            self.system.enemy_move_speed += 0.1;
        } else if is_speed_up && self.system.enemy_move_speed < 7.0 {
            self.system.enemy_move_speed += 0.05;
        } else if is_speed_up {
            self.system.enemy_move_speed += 0.025
        }
    }
    
    /// 敵移動速度調整のための、変数内容を返す関数
    fn enemy_move_speed(&self) -> f32 {
        self.system.enemy_move_speed
    }
    
    /// 敵を一定間隔ごとに増やす
    fn enemy_pop(&mut self) {
        // 今はとりあえず、4秒ごとに敵を1体増やす
        if self.system.frames % 240 == 0 {
            let tmp_n = self.enemy_pop_width();
            self.actor.add_e_block(
                etc::random_x(tmp_n),
                -50.0,
            );
        }
    }
    
    fn enemy_pop_width(&self) -> u32 {
        self.system.window_w - self.actor.template.e_block.width
    }
    
    /// 敵の当たり判定処理
    fn enemy_collision_check(&mut self) {
        // thread移行させるため安易にclone()
        let e_block_vec = self.actor.e_block.clone();
        let p_collision = self.actor.player.collision.clone();
        
        let is_crash = thread::spawn(move || {
            let mut out_bool = false;
            
            for i in 0..e_block_vec.len() {
                if e_block_vec[i].collision.is_overlap(&p_collision) {
                    out_bool = true;
                }
            }
            
            out_bool
        }).join().expect("is_crash handle開封時エラー");
        
        // この部分に衝突時の内容を書き加える
        if is_crash {
            // println!("{}, クラッシュ！", self.system.frames);
            self.system.is_game_over = true;
        }
    }
    
    /// デバッグ用のキー。用意しておいて、適当に書き換えて使う。
    fn debug_key(&mut self, input: &mut InputState) {
        if input.key_m {
            input.key_m = false;
            self.actor.add_e_block(
                etc::random_x(self.system.window_w),
                0.0,
            );
        }
    }
}

/// 縦軸あるいは横軸において、相反する移動結果を足して合わせる
/// 変数例: devide_move = move_up; add_move = move_down; 
fn key_move(divide_move: bool, add_move: bool) -> f32 {
    let n_devide = f32::from(divide_move as i8);
    let n_add = f32::from(add_move as i8); 
    
    -n_devide + n_add
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
        axis_value as f32 / 32768_f32
    } else {
        axis_value as f32 / 32768_f32
    };
    
    out_n
}
