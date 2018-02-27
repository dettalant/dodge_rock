/*-------------------------------
            core_state.rs

  ggezを使ってゲームシステム管理する部分を、
  CoreStateの形でまとめる。
  
  ゲーム内処理についてはgame_state.rsを、
  画面描画についてはview.rsを参照のこと。

-------------------------------*/ 

// use ggez::{ graphics };
use ggez::{ Context, GameResult};
use ggez::graphics::{ Point2 };
use ggez::event::{ EventHandler };

use assets;
use view;

pub struct CoreState {
    pub assets: assets::Assets,
    pub test_pos: Point2,
}

impl CoreState {
    pub fn new(ctx: &mut Context) -> GameResult<CoreState> {
        let test_pos = Point2::new(200.0, 200.0);
        Ok(CoreState {
            assets: assets::Assets::new(ctx)?,
            test_pos: test_pos,
        })
    }
}

impl EventHandler for CoreState {
    fn update(&mut self, _ctx: &mut Context) -> GameResult<()> {
        Ok(())
    }
    
    fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
        view::render_game(self, ctx)?;
        Ok(())
    }
}
