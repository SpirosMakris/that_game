extern crate specs;
use specs::prelude::*;

use ggez;
use ggez::graphics as gfx;
use ggez::{Context, GameResult};

use super::na;
use super::{CombatStats, Player};

use super::gamelog::GameLog;

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
        draw_health_bar(
            ctx,
            stats.hp,
            stats.max_hp,
            x as f32,
            y as f32,
            w as f32,
            h as f32,
            fg_color,
            bg_color,
        )?;
    }

    let x: i32 = GRID_TILE_SIZE * 2; // @TODO @HARDCODED
    let y: i32 = GRID_TILE_SIZE * 44; // @TODO @HARDCODED
    let max_y: i32 = GRID_TILE_SIZE * 49;   // @TODO @HARDCODED


    // Draw log
    let log = ecs.fetch::<GameLog>();
    draw_log(ctx, &log, x as f32, y as f32, max_y as f32)?;

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
        if width < 0.0 {
            width = 0.0;
        }
        width
    };

    let health_full_rect = gfx::Rect::new(x, y, w, h);
    let health_curr_rect = gfx::Rect::new(x, y, curr_health_width, h);

    let health_mesh = health_mb
        .rectangle(gfx::DrawMode::fill(), health_full_rect, bg_color)
        .rectangle(gfx::DrawMode::fill(), health_curr_rect, fg_color)
        .build(ctx)?;
    // Add the text overlay
    let health_text = gfx::Text::new(gfx::TextFragment {
        text: format!("{}/{}", curr_health, max_health),
        scale: Some(gfx::Scale::uniform(10.0)),
        ..Default::default()
    });

    gfx::queue_text(ctx, &health_text, na::Point2::new(x, y), None);

    // Render it
    // Health mesh
    gfx::draw(ctx, &health_mesh, gfx::DrawParam::default())?;
    // Health text
    gfx::draw_queued_text(
        ctx,
        gfx::DrawParam::default(),
        None,
        gfx::FilterMode::Linear,
    )?;

    Ok(())
}

// @TODO: Pull text settings to a struct var?

fn draw_log(ctx: &mut Context, log: &GameLog, x: f32, y: f32, max_y: f32) -> GameResult {
    let mut log_text = &mut gfx::Text::new(gfx::TextFragment {
        text: "Welcome to Rusty Roguelike".to_string(),
        scale: Some(gfx::Scale::uniform(10.0)),
        ..Default::default()
    });

    let mut y = y;    
    for entry in log.entries.iter().rev() {
        if y < max_y {
            // tmp_string = entry.clone();
            log_text = log_text.add(entry.clone());
            gfx::queue_text(ctx, &log_text, na::Point2::new(x, y), None );
        }
        y += (1 * GRID_TILE_SIZE) as f32;  // @TODO @HARDCODED 
    }
    
    gfx::draw_queued_text(ctx, gfx::DrawParam::default(), None, gfx::FilterMode::Linear)?;

    Ok(())
}
