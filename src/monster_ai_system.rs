extern crate specs;
use specs::prelude::*;
use super::{Viewshed, Monster, Name, Map, GridPosition};

extern crate rltk;

pub struct MonsterAISystem {}

impl<'a> System<'a> for MonsterAISystem {
  #[allow(clippy::type_complexity)]
  type SystemData = ( WriteExpect<'a, Map>,
                      ReadExpect<'a, rltk::Point>,  // Player pos
                      WriteStorage<'a, Viewshed>,
                      ReadStorage<'a, Monster>,
                      ReadStorage<'a, Name>,
                      WriteStorage<'a, GridPosition>);

  fn run(&mut self, data: Self::SystemData) {
    let (mut map, player_pos, mut viewshed, monster, name, mut position) = data;

    for (mut viewshed, _monster, name, mut pos) in (&mut viewshed, &monster, &name, &mut position).join() {

      if viewshed.visible_tiles.contains(&*player_pos) {
        println!("{} shouts insults", name.name);

        // Get a path to player
        let path = rltk::a_star_search(
          map.xy_idx(pos.x, pos.y) as i32, 
          map.xy_idx(player_pos.x, player_pos.y) as i32, 
          &mut *map
        );

        if path.success && path.steps.len() > 1 {
          pos.x = path.steps[1] as i32 % map.width;
          pos.y = path.steps[1] as i32 / map.width;
          viewshed.dirty = true;
        }
      }
    }
  }
}