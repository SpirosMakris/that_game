extern crate specs;
use super::{CombatStats, SufferDamage, Player};
use specs::prelude::*;

pub struct DamageSystem {}

impl<'a> System<'a> for DamageSystem {
    type SystemData = (WriteStorage<'a, CombatStats>, WriteStorage<'a, SufferDamage>);

    fn run(&mut self, data: Self::SystemData) {
        let (mut stats, mut damage) = data;

        for (mut stats, damage) in (&mut stats, &damage).join() {
            stats.hp -= damage.amount;
        }
        // We've processed all SufferDamage comps, so clear them
        damage.clear();
    }
}

pub fn delete_the_dead(ecs: &mut World) {
    let mut dead: Vec<Entity> = Vec::new();
    // Using a scope to make the borrow checker happy (at ecs.delete_entity)
    {

      let combat_stats = ecs.read_storage::<CombatStats>();
      let players = ecs.read_storage::<Player>();
      let entities = ecs.entities();
      
      for (entity, stats) in (&entities, &combat_stats).join() {
        if stats.hp < 1 {
          let player = players.get(entity);
          match player {
            None => dead.push(entity),  // Not the player, so delete it
            Some(_) => println!("You are dead!!!!"),
          }
        }
      }
    }

    for victim in dead {
        ecs.delete_entity(victim)
            .expect("Unable to delete dead entity");
    }
}
