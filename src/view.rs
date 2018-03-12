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
use ggez::graphics::{ self, Point2, Text};

use core_state::CoreState;

/// メインゲーム画面を描画
pub fn render_game(core: &mut CoreState, ctx: &mut Context) -> GameResult<()> {
    
    render_player(core, ctx)?;
    render_enemy(core, ctx)?;
    
    if env::var("GAME_ACTIVATE_MODE").unwrap() == "DEBUG_MODE" {
        debug_render(core, ctx)?;
    }
    
    Ok(())
}

/// プレイヤーを描画
fn render_player(core: &mut CoreState,
                 ctx: &mut Context) -> GameResult<()> {
    let player_pos = Point2::new(
        core.game_state.actor.player.x, 
        core.game_state.actor.player.y);
    
    graphics::draw(ctx, 
                   &core.assets.player_ship, 
                   player_pos,
                   0.0)?;
    Ok(())
}

/// 敵キャラクターを描画
fn render_enemy(core: &mut CoreState,
                ctx: &mut Context) -> GameResult<()> {
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

/// デバッグ引数がオンの時にだけ当たり判定を描画
fn debug_render(core: &mut CoreState,
                ctx: &mut Context) -> GameResult<()> {
    let actor = &core.game_state.actor;
    
    let p_col_rect = graphics::Rect::new(
        actor.player.x + 10.0,
        actor.player.y + 22.0,
        (actor.player.width - 20) as f32,
        (actor.player.height - 35) as f32,
    );

    let p_color = graphics::Color::from_rgba(0, 0, 255, 230);
    
    graphics::set_color(ctx, p_color)?;
    
    graphics::rectangle(ctx,
                        graphics::DrawMode::Fill,
                        p_col_rect)?;
    
    graphics::set_color(ctx, 
                        core.assets.dialog.default_color)?;
    
    Ok(())
}

pub fn render_game_over(core: &mut CoreState,
                        ctx: &mut Context) -> GameResult<()> {
    // foreground colorを変える
    graphics::set_color(ctx,
                        core.assets.dialog.go_box_color)?;
    
    // ダイアログボックスを表示
    graphics::rectangle(ctx,
                        graphics::DrawMode::Fill,
                        core.assets.dialog.go_box)?;
    
    // ダイアログボックス上テキストを描画
    render_game_over_dialog_text(core, ctx)?;
    
    // foreground colorを元に戻す
    graphics::set_color(ctx,
                        core.assets.dialog.default_color)?;
    Ok(())
}

fn render_game_over_dialog_text(core: &mut CoreState,
                                ctx: &mut Context) -> GameResult<()> {
    let (go_title_pos, go_score_pos, go_tip_pos) = game_over_dialog_text_pos(core);    
    
    graphics::set_color(ctx,
                        core.assets.dialog.black_color)?;
    
    graphics::draw(ctx,
                   &core.text.game_over_title,
                   go_title_pos,
                   0.0)?;

    graphics::draw(ctx,
                   &core.text.game_over_score,
                   go_score_pos,
                   0.0)?;
    
    render_result_score(core, ctx)?;
    
    draw_ml_text(ctx,
                 &core.text.game_over_tips,
                 go_tip_pos,
                 0.0)?;
    
    graphics::set_color(ctx,
                        core.assets.dialog.default_color)?;

    Ok(())
}

/// Point2指定がやけに長いので、別関数に分けておく
fn game_over_dialog_text_pos(core: &mut CoreState) -> (Point2, Point2, Vec<Point2>) {
    let (window_w, window_h) = (
        core.game_state.system.window_w as f32,
        core.game_state.system.window_h as f32
    );
    
    let go_title_pos = Point2::new(
        (window_w - core.text.game_over_title.width() as f32) / 2.0,
        (window_h - core.text.game_over_title.height() as f32 ) * 0.32,
    );
    
    let go_score_pos = Point2::new(
        (window_w - core.text.game_over_score.width() as f32 ) / 2.0,
        (window_h - core.text.game_over_score.height() as f32 ) * 0.42,
    );
    
    let go_tip_pos = calc_ml_text_pos(
        &core.text.game_over_tips,
        window_w,
        window_h,
        0.5,
        0.575,
    );
    
    (go_title_pos, go_score_pos, go_tip_pos)
}

fn render_result_score(core: &mut CoreState,
                       ctx: &mut Context) -> GameResult<()>{
    // 一ゲーム中に一度だけスコアを印字
    if !core.game_state.system.is_score_wrote {
        core.text.new_score(
            ctx,
            &core.assets.pixel_font,
            core.game_state.system.frames)?;
        core.game_state.system.is_score_wrote = true;
    }
    
    let (window_w, window_h) = (
        core.game_state.system.window_w as f32,
        core.game_state.system.window_h as f32
    );
    
    let go_score_num_pos = Point2::new(
        (window_w - core.text.game_over_score_num.width() as f32) / 2.0,
        (window_h - core.text.game_over_score_num.height() as f32 ) * 0.49,
    );
    
    graphics::draw(ctx,
                   &core.text.game_over_score_num,
                   go_score_num_pos,
                   0.0)?;
    
    Ok(())
}

/// 複数行に渡るテキストを描画するために、Vec<Point2>を作る
fn calc_ml_text_pos(in_vec: &Vec<Text>,
                    window_w: f32,
                    window_h: f32,
                    width_adjust: f32,
                    height_adjust: f32,) -> Vec<Point2> {
    // window_w: 画面の横サイズ
    // window_h: 画面の縦サイズ
    // width_adjust: 0.5で中央に表示
    // height_adjust: 0.5で中央に表示
    
    let mut out_vec = Vec::with_capacity(in_vec.len());
    let mut tmp_height = 0.0;
    
    // forを回して、Vec<Point2>を作り出す。
    // その際に、(以前の行の縦幅 + 予幅)を足して表示調整する
    // 横については一行目を基準に左寄せ。
    for i in 0..in_vec.len() {
        let tmp_pos = Point2::new(
            (window_w - in_vec[0].width() as f32) * width_adjust,
            (window_h - in_vec[i].height() as f32 + tmp_height) * height_adjust,
        );
       
        // 予幅を付けておいたほうが綺麗に表示できるはず  
        tmp_height += in_vec[i].height() as f32 + 20.0;
        out_vec.push(tmp_pos);
    }
    
    out_vec
}

/// 複数行にわたるテキストを描画する
fn draw_ml_text(ctx: &mut Context, 
                in_vec: &Vec<Text>,
                pos: Vec<Point2>,
                rotation: f32) -> GameResult<()> {
    for i in 0..in_vec.len() {
        graphics::draw(ctx,
                       &in_vec[i],
                       pos[i],
                       rotation)?;
    }
    
    Ok(())
}
