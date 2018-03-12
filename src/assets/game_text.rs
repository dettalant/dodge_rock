/*-------------------------------
            game_text.rs

  ゲーム内で表示するテキストを一挙に取ってくる
  Tomlから取得する予定
  
-------------------------------*/
use std::io::Result;

use ggez::graphics::{ Font, Text };
use ggez::{ Context, GameResult };
use toml;

use assets::Assets;
use etc;

#[derive(Clone, Debug, Deserialize)]
pub struct Source {
    title_logo: String,
    title_description: String,
    title_headline: String,
    title_tips: Vec<String>,
    game_over_title: String,
    game_over_score: String,
    game_over_tips: Vec<String>,
}

impl Source {
    fn new(assets: &Assets) -> Result<Self> {
        // ggez用にいじってるPathBufを正常な&strにする: 7れんさ
        let text_tmp_path = assets
            .show_map()
            .get("game_text.toml")
            .expect("HashMap.get()時のエラー")
            .strip_prefix("/")
            .expect("strip_prefix時のエラー")
            .to_str()
            .expect("PathBuf.to_str()時のエラー");
            
        let text_path = etc::easy_path_set(text_tmp_path);
        
        let tmp_vec = etc::File::read_to_vec(&text_path)?;
        
        let src_text: Source = toml::de::from_slice(&tmp_vec).expect("toml deserialize時のエラー");
        
        Ok(src_text)
    }
}

#[derive(Clone, Debug)]
pub struct GameText {
    pub title_logo: Text,
    pub title_description: Text,
    pub title_headline: Text,
    pub title_tips: Vec<Text>,
    pub game_over_title: Text,
    pub game_over_score: Text,
    pub game_over_score_num: Text,
    pub game_over_tips: Vec<Text>,
}

impl GameText {
    /// GameTextの生成
    pub fn new(ctx: &mut Context,
               assets: &Assets) -> GameResult<Self> {
        let src = Source::new(assets)?;
        
        let title_logo = Text::new(
            ctx,
            &src.title_logo,
            &assets.pixel_font_big,
        )?;
        
        let title_description = Text::new(
            ctx,
            &src.title_description,
            &assets.pixel_font_small,
        )?;
        
        let title_headline = Text::new(
            ctx,
            &src.title_headline,
            &assets.pixel_font_small,
        )?;
        
        let title_tips = GameText::from_array(
            ctx,
            &src.title_tips,
            &assets.pixel_font_small,
        )?;
        
        let game_over_title = Text::new(
            ctx,
            &src.game_over_title,
            &assets.pixel_font_big
        )?;
        
        let game_over_score = Text::new(
            ctx,
            &src.game_over_score,
            &assets.pixel_font
        )?;
        
        // 後で書き換えるものなので、適当にclone()しておく
        // ほんとは空テキストを出力できる機能がggezにあるべきなのよ。
        let game_over_score_num = game_over_score.clone();
        
        let game_over_tips = GameText::from_array(
            ctx,
            &src.game_over_tips,
            &assets.pixel_font,
        )?;
        
        Ok(GameText {
            title_logo: title_logo,
            title_description: title_description,
            title_headline: title_headline,
            title_tips: title_tips,
            game_over_title: game_over_title,
            game_over_score: game_over_score,
            game_over_score_num: game_over_score_num,
            game_over_tips: game_over_tips,
        })
    }
    
    // その時のフレーム数からスコアを計算する
    pub fn new_score(&mut self,
                     ctx: &mut Context,
                     font: &Font,
                     score: usize) -> GameResult<()> {
        let tmp_t = format!("**{}**", score);
        
        let out_t = Text::new(
            ctx,
            &tmp_t,
            font,
        )?;
        
        self.game_over_score_num = out_t;
        
        Ok(())
    }
    
    fn from_array(ctx: &mut Context,
                  in_vec: &Vec<String>,
                  font: &Font) -> GameResult<Vec<Text>> {
        let mut out_vec = Vec::with_capacity(in_vec.len());
        
        for li in in_vec {
            let tmp_t = Text::new(
                ctx,
                &li,
                font
            )?;
            
            out_vec.push(tmp_t);
        }
        
        Ok(out_vec)
    }
}

