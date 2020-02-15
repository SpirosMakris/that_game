extern crate specs;
use specs::prelude::*;
use super::{GridPosition, Viewshed, Map};

extern crate rltk;
use rltk::{field_of_view, Point};

pub struct VisibilitySystem {}

impl<'a> System<'a> for VisibilitySystem {
  type SystemData = ( ReadExpect<'a, Map>,
                      WriteStorage<'a, Viewshed>,
                      WriteStorage<'a, GridPosition>);
  
  fn run(&mut self, data: Self::SystemData) {
    // Destructure system data
    let (map, mut viewshed, pos) = data;

    for (viewshed, pos) in (&mut viewshed, &pos).join() {
      // Clear visible tiles in this Viewshed
      viewshed.visible_tiles.clear();

      // Calculate visible tiles from current position
      viewshed.visible_tiles = field_of_view(Point::new(pos.x, pos.y), viewshed.range, &*map);

      // Filter out visible tiles that are outside map boundaries
      viewshed.visible_tiles.retain(|p| {
        p.x > 0 && p.x < map.width - 1 && p.y > 0 && p.y < map.height - 1
      });

    }
  }
}