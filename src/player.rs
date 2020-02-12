use ggez::input::keyboard;
use ggez::event::KeyCode;
use ggez::Context;

use specs::prelude::*;

use super::{GridPosition, Player, TileType, xy_idx, State};

use std::cmp::{min, max};

// @TODO: Not refactoring for RLT 1.3. Just moved stuff here!!

pub fn try_move_player(delta_x: i32, delta_y: i32, ecs: &mut World) {
  let mut positions = ecs.write_storage::<GridPosition>();
  let mut players = ecs.write_storage::<Player>();
  let map = ecs.fetch::<Vec<TileType>>();

  for (_player, pos) in (&mut players, &mut positions).join() {
      let destination_idx = xy_idx(pos.x + delta_x, pos.y + delta_y);
      if map[destination_idx] != TileType::Wall {
          pos.x = min(79, max(0, pos.x + delta_x));
          pos.y = min(49, max(0, pos.y + delta_y));
      }
  }
}

pub fn player_input(gs: &mut State, ctx: &Context) {
  let pressed_keys = keyboard::pressed_keys(ctx);
  if pressed_keys.is_empty() { return };
  
  if keyboard::is_key_pressed(ctx, KeyCode::Left) {
      try_move_player(-1, 0, &mut gs.ecs);
  }

  if keyboard::is_key_pressed(ctx, KeyCode::Right) {
      try_move_player(1, 0, &mut gs.ecs);
  }

  if keyboard::is_key_pressed(ctx, KeyCode::Up) {
      try_move_player(0, -1, &mut gs.ecs);
  }

  if keyboard::is_key_pressed(ctx, KeyCode::Down) {
      try_move_player(0, 1 , &mut gs.ecs);
  }
}