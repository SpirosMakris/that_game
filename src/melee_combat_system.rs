extern crate specs;
use super::{CombatStats, Name, SufferDamage, WantsToMelee};
use specs::prelude::*;

pub struct MeleeCombatSystem {}

impl<'a> System<'a> for MeleeCombatSystem {
    type SystemData = (
        Entities<'a>,
        WriteStorage<'a, WantsToMelee>,
        ReadStorage<'a, Name>,
        ReadStorage<'a, CombatStats>,
        WriteStorage<'a, SufferDamage>,
    );

    fn run(&mut self, data: Self::SystemData) {
        let (entities, mut wants_melee, names, combat_stats, mut inflict_damage) = data;

        for (_entity, wants_melee, name, stats) in
            (&entities, &wants_melee, &names, &combat_stats).join()
        {
            // If we are NOT dead
            if stats.hp > 0 {
                // Get the combat stats of our target
                let target_stats = combat_stats.get(wants_melee.target).unwrap();
                // If our target is NOT dead
                if target_stats.hp > 0 {
                    let target_name = names.get(wants_melee.target).unwrap();

                    let damage = i32::max(0, stats.power - target_stats.defense);

                    if damage == 0 {
                        println!("{} is unable to hurt {}", &name.name, &target_name.name);
                    } else {
                        println!(
                            "{} hits {}, for {} hp.",
                            &name.name, &target_name.name, damage
                        );
                        inflict_damage
                            .insert(wants_melee.target, SufferDamage { amount: damage })
                            .expect("Unable to insert SufferDamage comp");
                    }
                }
            }
        }
        // We have proccesed all wants to melee components so clear them
        wants_melee.clear();
    }
}
