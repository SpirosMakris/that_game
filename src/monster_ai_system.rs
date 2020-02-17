extern crate specs;
use specs::prelude::*;
use super::{Viewshed, Map, Monster};

extern crate rltk;
use rltk::{field_of_view, Point};

pub struct MonsterAISystem {}

impl<'a> System<'a> for MonsterAISystem {
  type SystemData = ( ReadExpect<'a, rltk::Point>,
                      ReadStorage<'a, Viewshed>,
                      ReadStorage<'a, Monster>);

  fn run(&mut self, data: Self::SystemData) {
    let (player_pos, viewshed, monster) = data;

    for (viewshed, _monster) in (&viewshed, &monster).join() {
      if viewshed.visible_tiles.contains(&*player_pos) {
        println!("Monster shouts insults");
      }
    }
  }
}