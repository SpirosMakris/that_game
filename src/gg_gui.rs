extern crate specs;
use specs::prelude::*;

use ggez;
use ggez::graphics as gfx;
use ggez::{Context, GameResult};

use super::{CombatStats, Player};

use crate::map::GRID_TILE_SIZE;

pub fn draw_ui(ecs: &World, ctx: &mut Context) -> GameResult {
    // Uses meshbuilder even for simple rects

    let mut gui_mb = gfx::MeshBuilder::new();

    // Draw the bottom gui rect
    let x = 0;
    let y = GRID_TILE_SIZE * 43; // @TODO @HARDCODED
    let h = GRID_TILE_SIZE * (7 - 1); // @TODO @HARDCODED
    let w = GRID_TILE_SIZE * (80 - 1); // @TODO @HARDCODED
    let bottom_gui_rect = gfx::Rect::new_i32(x, y, w, h);

    let bottom_color = gfx::Color::new(0.4, 0.4, 0.4, 0.5);

    // Apply to gui mesh
    let gui_mesh = gui_mb
        .rectangle(gfx::DrawMode::fill(), bottom_gui_rect, bottom_color)
        .build(ctx)?;

    // Render it
    gfx::draw(ctx, &gui_mesh, gfx::DrawParam::default())?;

    // Draw the health bar
    let combat_stats = ecs.read_storage::<CombatStats>();
    let players = ecs.read_storage::<Player>();
    let x = GRID_TILE_SIZE * 28; // @TODO @HARDCODED
    let y = GRID_TILE_SIZE * 43; // @TODO @HARDCODED
    let w = GRID_TILE_SIZE * (51 - 43); // @TODO @HARDCODED
    let h = GRID_TILE_SIZE; // @TODO @HARDCODED

    let fg_color = gfx::Color::new(1.0, 0.1, 0.1, 1.0);
    let bg_color = gfx::Color::new(0.1, 0.1, 0.1, 1.0);
    for (_player, stats) in (&players, &combat_stats).join() {
      draw_health_bar(ctx, stats.hp, stats.max_hp, x as f32, y as f32, w as f32, h as f32, fg_color, bg_color)?;
    }

    Ok(())
}

fn draw_health_bar(
    ctx: &mut Context,
    curr_health: i32,
    max_health: i32,
    x: f32,
    y: f32,
    w: f32,
    h: f32,
    fg_color: gfx::Color,
    bg_color: gfx::Color,
) -> GameResult {
    let mut health_mb = gfx::MeshBuilder::new();

    // Draw the health bar
    let curr_health_width = {
      let mut width = w * curr_health as f32 / max_health as f32;
      if width < 0.0 { width = 0.0; }
      width
    };

    let health_full_rect = gfx::Rect::new(x, y, w, h);
    let health_curr_rect = gfx::Rect::new(x, y, curr_health_width, h);

    let health_mesh = health_mb
      .rectangle(gfx::DrawMode::fill(), health_full_rect, bg_color)
      .rectangle(gfx::DrawMode::fill(), health_curr_rect, fg_color)
      .build(ctx)?;
    
    // Render it

    gfx::draw(ctx, &health_mesh, gfx::DrawParam::default())?;

    Ok(())
}
