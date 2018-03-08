/*-------------------------------
            view.rs

  画面描画関連を一つにまとめる
  
  まとめた関数を、core_state.rsのggez EventHandlerに渡す
  ゲーム内処理についてはgame_state.rsを参照のこと
    
  * render_game  : ゲームの状況に合わせて、適切な部分を描画するおまとめ関数
  * render_player: プレイヤー周りを描画する
-------------------------------*/ 
use ggez::{ Context, GameResult };
use ggez::graphics::{ self, Point2 };

use core_state::CoreState;

pub fn render_game(core: &mut CoreState, ctx: &mut Context) -> GameResult<()> {
    graphics::clear(ctx);
    
    render_player(core, ctx)?;
    render_enemy(core, ctx)?;
    
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

fn render_enemy(core: &mut CoreState, ctx: &mut Context) -> GameResult<()> {
    // いちいち書くのがだるいので、短縮ネームを変数束縛
    let e_block = &core.game_state.actor.e_block;
    
    for li in e_block {
        let e_block_pos = Point2::new(
            li.x,
            li.y,
        );
        
        graphics::draw(ctx,
                       &core.assets.enemy_block,
                       e_block_pos,
                       0.0)?;
    }
    
    Ok(())
}
