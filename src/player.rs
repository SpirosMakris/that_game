use ggez::input::keyboard;
use ggez::event::KeyCode;
use ggez::Context;

use specs::prelude::*;

use super::{GridPosition, Player, TileType, Map, State, Viewshed};

use std::cmp::{min, max};

// @TODO: Not refactoring for RLT 1.3. Just moved stuff here!!

pub fn try_move_player(delta_x: i32, delta_y: i32, ecs: &mut World) {
  let mut positions = ecs.write_storage::<GridPosition>();
  let mut players = ecs.write_storage::<Player>();
  let mut viewsheds = ecs.write_storage::<Viewshed>();
  let map = ecs.fetch::<Map>();

  for (_player, pos, viewshed) in (&mut players, &mut positions, &mut viewsheds).join() {
      let destination_idx = map.xy_idx(pos.x + delta_x, pos.y + delta_y);
      if map.tiles[destination_idx] != TileType::Wall {
          pos.x = min(79, max(0, pos.x + delta_x));
          pos.y = min(49, max(0, pos.y + delta_y));

          // We've moved so mark our viewshed as dirty to recalculate
          viewshed.dirty = true;
      }
  }
}

pub fn player_input(gs: &mut State, ctx: &Context) {
  let pressed_keys = keyboard::pressed_keys(ctx);
  if pressed_keys.is_empty() { return };
  
  if keyboard::is_key_pressed(ctx, KeyCode::Left) ||
     keyboard::is_key_pressed(ctx, KeyCode::Numpad4) ||
     keyboard::is_key_pressed(ctx, KeyCode::H) {
      try_move_player(-1, 0, &mut gs.ecs);
  }

  if keyboard::is_key_pressed(ctx, KeyCode::Right) ||
     keyboard::is_key_pressed(ctx, KeyCode::Numpad6) ||
     keyboard::is_key_pressed(ctx, KeyCode::L) {
      try_move_player(1, 0, &mut gs.ecs);
  }

  if keyboard::is_key_pressed(ctx, KeyCode::Up) ||
     keyboard::is_key_pressed(ctx, KeyCode::Numpad8) ||
     keyboard::is_key_pressed(ctx, KeyCode::K) {
      try_move_player(0, -1, &mut gs.ecs);
  }

  if keyboard::is_key_pressed(ctx, KeyCode::Down) ||
     keyboard::is_key_pressed(ctx, KeyCode::Numpad2) ||
     keyboard::is_key_pressed(ctx, KeyCode::J) {
      try_move_player(0, 1 , &mut gs.ecs);
  }
}