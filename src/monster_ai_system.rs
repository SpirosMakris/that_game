extern crate specs;
use specs::prelude::*;
use super::{Viewshed, Monster, Name};

extern crate rltk;

pub struct MonsterAISystem {}

impl<'a> System<'a> for MonsterAISystem {
  type SystemData = ( ReadExpect<'a, rltk::Point>,
                      ReadStorage<'a, Viewshed>,
                      ReadStorage<'a, Monster>,
                      ReadStorage<'a, Name>);

  fn run(&mut self, data: Self::SystemData) {
    let (player_pos, viewshed, monster, name) = data;

    for (viewshed, _monster, name) in (&viewshed, &monster, &name).join() {
      if viewshed.visible_tiles.contains(&*player_pos) {
        println!("{} shouts insults", name.name);
      }
    }
  }
}