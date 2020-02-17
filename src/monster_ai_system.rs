extern crate specs;
use specs::prelude::*;
use super::{Viewshed, GridPosition, Map, Monster};

extern crate rltk;
use rltk::{field_of_view, Point};

pub struct MonsterAISystem {}

impl<'a> System<'a> for MonsterAISystem {
  type SystemData = ( ReadStorage<'a, Viewshed>,
                      ReadStorage<'a, GridPosition>,
                      ReadStorage<'a, Monster>);

  fn run(&mut self, data: Self::SystemData) {
    let (viewshed, pos, monster) = data;

    for (viewshed, pos, _monster) in (&viewshed, &pos, &monster).join() {
      println!("Monster considers their own existence");
    }
  }
}