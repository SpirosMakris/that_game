extern crate specs;
use super::{GridPosition, Map, Player, Viewshed};
use specs::prelude::*;

extern crate rltk;
use rltk::{field_of_view, Point};

pub struct VisibilitySystem {}

impl<'a> System<'a> for VisibilitySystem {
    type SystemData = (
        WriteExpect<'a, Map>,
        Entities<'a>,
        WriteStorage<'a, Viewshed>,
        WriteStorage<'a, GridPosition>,
        ReadStorage<'a, Player>,
    );

    fn run(&mut self, data: Self::SystemData) {
        // Destructure system data
        let (mut map, entities, mut viewshed, pos, player) = data;

        for (ent, viewshed, pos) in (&entities, &mut viewshed, &pos).join() {
            // Recalculate viewshed for this position if needed
            if viewshed.dirty {
                // Clear visible tiles in this Viewshed
                viewshed.visible_tiles.clear();

                // Calculate visible tiles from current position
                viewshed.visible_tiles =
                    field_of_view(Point::new(pos.x, pos.y), viewshed.range, &*map);

                // Filter out visible tiles that are outside map boundaries
                viewshed
                    .visible_tiles
                    .retain(|p| p.x > 0 && p.x < map.width - 1 && p.y > 0 && p.y < map.height - 1);

                // If this is a player update map to reveal what they see
                let _p: Option<&Player> = player.get(ent);
                if let Some(_p) = _p {
                    // Clear the previously visible tiles, they have changed
                    for t in map.visible_tiles.iter_mut() {
                        *t = false
                    }

                    for vis in viewshed.visible_tiles.iter() {
                        let idx = map.xy_idx(vis.x, vis.y);
                        // Add (but not cleared)
                        map.revealed_tiles[idx] = true;
                        // Add (cleared just above)
                        map.visible_tiles[idx] = true;
                    }
                }
            }
        }
    }
}
