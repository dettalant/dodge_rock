/*-------------------------------
            view.rs

  画面描画関連を一つにまとめる
  
  まとめた関数を、core_state.rsのggez EventHandlerに渡す
  ゲーム内処理についてはgame_state.rsを参照のこと

-------------------------------*/ 
use ggez::{ Context, GameResult };
use ggez::graphics::{ self, Point2 };

use core_state::CoreState;

pub fn render_game(core: &mut CoreState, ctx: &mut Context) -> GameResult<()> {
    graphics::clear(ctx);
    
    render_player(core, ctx)?;
    
    graphics::present(ctx);
    
    Ok(())
}

fn render_player(core: &mut CoreState, ctx: &mut Context) -> GameResult<()> {
    let player_pos = Point2::new(
        core.game_state.actor.player.x, 
        core.game_state.actor.player.y);
    
    graphics::draw(ctx, 
                   &core.assets.player_ship, 
                   player_pos,
                   0.0)?;
    Ok(())
}
