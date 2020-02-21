extern crate specs;
use specs::prelude::*;

use ggez;
use ggez::graphics as gfx;
use ggez::{Context, GameResult};

use crate::map::GRID_TILE_SIZE;

pub fn draw_ui(ecs: &World, ctx: &mut Context) -> GameResult {
    // Uses meshbuilder even for simple rects

    let mut gui_meshbuilder = gfx::MeshBuilder::new();

    // Draw the bottom gui rect
    let x = 0;
    let y = GRID_TILE_SIZE * 43;  // @TODO @HARDCODED
    let h = GRID_TILE_SIZE * (7 - 1); // @TODO @HARDCODED
    let w = GRID_TILE_SIZE * (80 -1);  // @TODO @HARDCODED
    let bottom_gui_rect = gfx::Rect::new_i32(x, y, w, h);

    let bottom_color = gfx::Color::new(0.4, 0.4, 0.4, 0.5);
    
    // Apply to gui mesh
    let gui_mesh= gui_meshbuilder.rectangle(gfx::DrawMode::fill(), bottom_gui_rect, bottom_color)
      .build(ctx)?;
    
    // Render it
    gfx::draw(ctx, &gui_mesh, gfx::DrawParam::default())?;

    Ok(())
}
