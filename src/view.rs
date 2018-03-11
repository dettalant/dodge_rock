/*-------------------------------
            view.rs

  画面描画関連を一つにまとめる
  
  まとめた関数を、core_state.rsのggez EventHandlerに渡す
  ゲーム内処理についてはgame_state.rsを参照のこと
    
  * render_game()  : ゲームの状況に合わせて、適切な部分を描画するおまとめ関数
  * render_player(): プレイヤー周りを描画する
  * render_enemy() :
  * debug_render() :
-------------------------------*/ 
use std::env;

use ggez::{ Context, GameResult };
use ggez::graphics::{ self, Point2 };

use core_state::CoreState;

pub fn render_game(core: &mut CoreState, ctx: &mut Context) -> GameResult<()> {
    graphics::clear(ctx);
    
    render_player(core, ctx)?;
    render_enemy(core, ctx)?;
    
    if env::var("GAME_ACTIVATE_MODE").unwrap() == "DEBUG_MODE" {
        debug_render(core, ctx)?;
    }
    
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

fn debug_render(core: &mut CoreState, ctx: &mut Context) -> GameResult<()> {
    let actor = &core.game_state.actor;
    
    let p_col_rect = graphics::Rect::new(
        actor.player.x + 10.0,
        actor.player.y + 22.0,
        (actor.player.width - 20) as f32,
        (actor.player.height - 35) as f32,
    );
    
    let default_color = graphics::Color::new(1.0, 1.0, 1.0, 1.0);
    let p_color = graphics::Color::from_rgba(0, 0, 255, 230);
    
    graphics::set_color(ctx, p_color)?;
    
    graphics::rectangle(
        ctx,
        graphics::DrawMode::Fill,
        p_col_rect,
    )?;
    
    graphics::set_color(ctx, default_color)?;
    
    Ok(())
}
