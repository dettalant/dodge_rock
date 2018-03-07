/*-------------------------------
            audio.rs

  音楽・効果音を流すための部分
  主にassetsフォルダから読み込んで使いたい
  
  （まだWIP段階）
-------------------------------*/
use std::path::PathBuf;
use std::collections::HashMap;
use std::sync::{ mpsc, Arc };
use std::thread;

use ggez::audio;
use ggez::{ Context, GameResult };

use input_state::InputState;

/// Gameで使うBGMはすべてここに溜める
pub struct AudioData {
    pub test_bgm: audio::Source,
    pub no_sound: audio::Source,
}

impl AudioData {
    pub fn new(ctx: &mut Context, 
               assets_map: HashMap<String, PathBuf>) -> GameResult<Self>{
        let test_bgm = audio::Source::new(
            ctx,
            assets_map.get("neo_honobono_jinja.ogg").unwrap(),
        )?;
        
        let no_sound = audio::Source::new(
            ctx,
            assets_map.get("no_sound_3s.wav").unwrap(),
        )?;
        
        Ok(AudioData {
            test_bgm: test_bgm,
            no_sound: no_sound,
        })
    }
}

pub struct GameAudio {
    /// 扱いやすくするために所有権を手に入れる
    audio: AudioData,
    /// BGMが現在再生されているか
    is_bgm: bool,
    /// 現在かけてる曲
    current_bgm: i32,
    /// どの曲をかけるのかを選択するリモコン
    ctrl_sender: mpsc::Sender<i32>,
}

impl GameAudio {
    /// 扱いやすい形に整える
    pub fn new<'a>(ctx: &mut Context, 
                   assets_map: HashMap<String, PathBuf>) -> GameResult<Self> {
        let audio = AudioData::new(ctx, assets_map)?;
        let (tx, _rx) = mpsc::channel::<i32>();
        
        Ok(GameAudio {
            audio: audio,
            is_bgm: false,
            current_bgm: 0,
            ctrl_sender: tx,
        })
    }
    
    ///// マルチスレッドにするために、曲をかける度に別のやつへと投げる
    //pub fn play_bgm(&mut self) {
        
        //let (tx, rx) = mpsc::channel::<audio::Source>();
        //let tmp_m = self.audio.test_bgm.clone();
        //tx.send().unwrap();
        
        //thread::spawn(move || {
            //let tmp_m = rx.recv().expect("BGM受信エラー");
            //tmp_m.play().expect("BGM再生時のエラー");
            
            //loop {
                //let ctrl_receiver = rx.recv().expect("BGM再生時、コントロール受信エラー");
                
                //match ctrl_receiver {
                    //0 => (),
                    //1 => tmp_m.stop(),
                    //_ => (),
                //}
            //}
        //});
        
    //}
    
    pub fn track_ctrl(&self, num: i32) { 
        self.ctrl_sender.send(num).expect("BGMコントローラー送信エラー");
    }
    
    /// bgmがかかってるかどうかを取得する
    pub fn is_bgm(&self) -> bool { self.is_bgm }
    
    /// 現在かかってる曲番号を表示する
    pub fn current_bgm(&self) -> i32 { self.current_bgm }
    
    pub fn bgm_tuner(&self, input: &InputState) {
        if input.key_d {
            self.ctrl_sender.send(1).unwrap();
            println!("ctrl_senderに1を送ったよ");
        } else if input.key_m {
            self.ctrl_sender.send(0).unwrap();
            println!("ctrl_senderに0を送ったよ");
        }
    }
}
