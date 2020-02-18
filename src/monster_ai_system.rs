extern crate specs;
use super::{GridPosition, Map, Monster, Viewshed, WantsToMelee};
use specs::prelude::*;

extern crate rltk;

pub struct MonsterAISystem {}

impl<'a> System<'a> for MonsterAISystem {
    #[allow(clippy::type_complexity)]
    type SystemData = (
        WriteExpect<'a, Map>,
        ReadExpect<'a, rltk::Point>, // Player pos
        ReadExpect<'a, Entity>,
        Entities<'a>,
        WriteStorage<'a, Viewshed>,
        ReadStorage<'a, Monster>,
        WriteStorage<'a, GridPosition>,
        WriteStorage<'a, WantsToMelee>,
    );

    fn run(&mut self, data: Self::SystemData) {
        let (
            mut map,
            player_pos,
            player_entity,
            entities,
            mut viewshed,
            monster,
            mut position,
            mut wants_to_melee,
        ) = data;

        for (entity, mut viewshed, _monster, mut pos) in
            (&entities, &mut viewshed, &monster, &mut position).join()
        {
            let distance = rltk::DistanceAlg::Pythagoras
                .distance2d(rltk::Point::new(pos.x, pos.y), *player_pos);
            if distance < 1.5 {
                // Attack goes here
                wants_to_melee
                    .insert(
                        entity,
                        WantsToMelee {
                            target: *player_entity,
                        },
                    )
                    .expect("Unable to insert attack");
            }

            if viewshed.visible_tiles.contains(&*player_pos) {
                // Get a path to player so we can follow him

                let path = rltk::a_star_search(
                    map.xy_idx(pos.x, pos.y) as i32,
                    map.xy_idx(player_pos.x, player_pos.y) as i32,
                    &mut *map,
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
