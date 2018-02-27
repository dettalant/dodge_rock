/*-------------------------------
            view.rs

  画面描画関連を一つにまとめる
  
  まとめた関数を、core_state.rsのggez EventHandlerに渡す
  ゲーム内処理についてはgame_state.rsを参照のこと

-------------------------------*/ 
use ggez::{ graphics };
use ggez::{ Context, GameResult };

use core_state::CoreState;

pub fn render_game(core: &mut CoreState, ctx: &mut Context) -> GameResult<()> {
    graphics::clear(ctx);
    
    graphics::draw(ctx, 
                   &core.assets.player_ship, 
                   core.test_pos,
                   0.0)?;
    
    graphics::present(ctx);
    
    Ok(())
}
