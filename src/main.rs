use specs::prelude::*;

#[macro_use]
extern crate specs_derive;

// use rltk::{Console, GameState, Rltk, RGB, VirtualKeyCode};

use ggez;
use ggez::event;
use ggez::graphics as gfx;
use ggez::nalgebra as na;
use ggez::timer;
use ggez::{Context, GameResult};

mod components;
pub use components::*;
mod rect32;
pub use rect32::Rect32;
mod player;
use player::*;
mod map;
pub use map::*;

mod visibility_system;
use visibility_system::VisibilitySystem;
mod monster_ai_system;
use monster_ai_system::MonsterAISystem;
mod map_indexing_system;
use map_indexing_system::MapIndexingSystem;
mod melee_combat_system;
use melee_combat_system::MeleeCombatSystem;
mod damage_system;
use damage_system::DamageSystem;

// mod imgui_wrapper;
// use imgui_wrapper::ImGuiWrapper;

// GAME STATE

#[derive(PartialEq, Copy, Clone)]
pub enum RunState {
    AwaitingInput,
    PreRun,
    PlayerTurn,
    MonsterTurn,
}

pub struct State {
    pub ecs: World,
    // imgui_wrapper: ImGuiWrapper,
}

impl State {
    fn run_systems(&mut self) {
        // Run visibility system
        let mut vis = VisibilitySystem {};
        vis.run_now(&self.ecs);

        // Run monster AI system
        let mut mob = MonsterAISystem {};
        mob.run_now(&self.ecs);

        // Run the map indexing system
        let mut mapindex = MapIndexingSystem {};
        mapindex.run_now(&self.ecs);

        // Run the melee combat system
        let mut melee = MeleeCombatSystem {};
        melee.run_now(&self.ecs);

        // Run damage system
        let mut damage = DamageSystem {};
        damage.run_now(&self.ecs);

        // Update world after running systems
        self.ecs.maintain();
    }
}

impl event::EventHandler for State {
    fn update(&mut self, ctx: &mut Context) -> GameResult {
        if timer::ticks(ctx) % 100 == 0 {
            println!("Dt frame time: {:?}", timer::delta(ctx));
            println!("Average FPS: {}", timer::fps(ctx));
        }

        // 'Extract' current runstate from ECS
        let mut newrunstate;
        {
            let runstate = self.ecs.fetch::<RunState>();
            newrunstate = *runstate;
        }
        
        // Match it
        match newrunstate {
            RunState::PreRun => {
                self.run_systems();
                newrunstate = RunState::AwaitingInput;
            },
            RunState::AwaitingInput => {
                // Any new state change is done in player input
                newrunstate = player_input(self, ctx);

            },
            RunState::PlayerTurn => {
                self.run_systems();
                newrunstate = RunState::MonsterTurn;
            },
            RunState::MonsterTurn => {
                self.run_systems();
                newrunstate = RunState::AwaitingInput;
            }
        }

        // Re-insert modified RunState into ECS
        {
            let mut runstate_writer = self.ecs.write_resource::<RunState>();
            *runstate_writer = newrunstate;
        }

        // match self.runstate {
        //     RunState::Running => {
        //         self.run_systems();
        //         self.runstate = RunState::Waiting;
        //     }
        //     RunState::Waiting => {
        //         self.runstate = player_input(self, ctx);
        //     }
        // }

        // Delete dead entities
        damage_system::delete_the_dead(&mut self.ecs);

        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        gfx::clear(ctx, [0.1, 0.2, 0.3, 1.0].into());

        // Render our map
        draw_map(&self.ecs, ctx)?;

        // RENDER MONSTERS
        let positions = self.ecs.read_storage::<GridPosition>();
        let renderables = self.ecs.read_storage::<Renderable>();
        let map = self.ecs.fetch::<Map>();

        for (pos, render) in (&positions, &renderables).join() {
            let idx = map.xy_idx(pos.x, pos.y);
            if map.visible_tiles[idx] {
                let circle = gfx::Mesh::new_circle(
                    ctx,
                    gfx::DrawMode::fill(),
                    na::Point2::new(0.0, 0.0),
                    10.0,
                    2.0,
                    render.color,
                )?;

                gfx::draw(
                    ctx,
                    &circle,
                    (na::Point2::new(
                        (pos.x * GRID_TILE_SIZE) as f32 + (GRID_TILE_SIZE / 2) as f32,
                        (pos.y * GRID_TILE_SIZE) as f32 + (GRID_TILE_SIZE / 2) as f32,
                    ),),
                )?;
            }
        }

        gfx::present(ctx)?;
        Ok(())
    }
}

fn main() -> GameResult {
    // @TODO: Screen dims to use for (80 x 50 , tile size 16) = 1280 x 800
    let cb = ggez::ContextBuilder::new("THAT GAME - super simple", "Spiros Makris");
    let (ctx, event_loop) = &mut cb.build()?;
    
    // Create State with ECS world in it.
    let mut gs = State {
        ecs: World::new(),
        // imgui_wrapper: ImGuiWrapper::new(&mut ctx) ,
    };

    // Register components
    gs.ecs.register::<GridPosition>();
    gs.ecs.register::<Renderable>();
    gs.ecs.register::<Player>();
    gs.ecs.register::<Viewshed>();
    gs.ecs.register::<Monster>();
    gs.ecs.register::<Name>();
    gs.ecs.register::<BlocksTile>();
    gs.ecs.register::<CombatStats>();
    gs.ecs.register::<WantsToMelee>();
    gs.ecs.register::<SufferDamage>();

    // Add a map to ECS resources
    // and placelace player in the center of 1st room
    let map: Map = Map::new_map_rooms_and_corridors();
    let (player_x, player_y) = map.rooms[0].center();

    // Create player
    let player_entity = gs
        .ecs
        .create_entity()
        .with(GridPosition {
            x: player_x,
            y: player_y,
        })
        .with(Renderable {
            color: gfx::Color::new(0., 1., 0., 1.),
        })
        .with(Player {})
        .with(Viewshed {
            visible_tiles: Vec::new(),
            range: 8,
            dirty: true,
        })
        .with(Name {
            name: "Player".to_string(),
        })
        .with(CombatStats {
            max_hp: 30,
            hp: 30,
            defense: 2,
            power: 5,
        })
        .build();

    // Add some monsters
    let mut rng = rltk::RandomNumberGenerator::new();

    for (i, room) in map.rooms.iter().skip(1).enumerate() {
        let (x, y) = room.center();

        let color: gfx::Color;
        let name: String;

        let roll = rng.roll_dice(1, 2);

        match roll {
            1 => {
                color = gfx::Color::new(1.0, 0.0, 0.75, 1.0);
                name = "Goblin".to_string();
            }

            _ => {
                color = gfx::Color::new(1.0, 0.0, 0.1, 1.0);
                name = "Orc".to_string();
            }
        }

        gs.ecs
            .create_entity()
            .with(GridPosition { x, y })
            .with(Renderable { color })
            .with(Viewshed {
                visible_tiles: Vec::new(),
                range: 8,
                dirty: true,
            })
            .with(Monster {})
            .with(Name {
                name: format!("{} #{}", &name, i),
            })
            .with(BlocksTile {})
            .with(CombatStats {
                max_hp: 16,
                hp: 16,
                defense: 1,
                power: 4,
            })
            .build();
    }

    // INSERT RESOURCES
    gs.ecs.insert(map);
    gs.ecs.insert(rltk::Point::new(player_x, player_y)); // @TODO: Should this be an rltk::Point or something else?
    gs.ecs.insert(player_entity);
    gs.ecs.insert(RunState::PreRun);

    

    event::run(ctx, event_loop, &mut gs)
}
