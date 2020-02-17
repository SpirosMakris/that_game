use specs::prelude::*;

#[macro_use]
extern crate specs_derive;

// use rltk::{Console, GameState, Rltk, RGB, VirtualKeyCode};



use ggez;
use ggez::event;
use ggez::graphics as gfx;
use ggez::nalgebra as na;
use ggez::{Context, GameResult};
use ggez::timer;



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


// GAME STATE

pub struct State {
    ecs: World,
}

impl State {
    fn run_systems(&mut self) {
        // let mut lw = LeftWalkerSys{};
        // lw.run_now(&self.ecs);

        // Run visibility system
        let mut vis = VisibilitySystem{};
        vis.run_now(&self.ecs);

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

        player_input(self, ctx);
        self.run_systems();
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
                    render.color
                )?;
    
                gfx::draw(ctx, &circle, (na::Point2::new( (pos.x * GRID_TILE_SIZE) as f32 + (GRID_TILE_SIZE / 2) as f32, (pos.y * GRID_TILE_SIZE) as f32 + (GRID_TILE_SIZE / 2) as f32), ))?;
            }
        }

        gfx::present(ctx)?;
        Ok(())
    }
}

// impl GameState for State {
//     fn tick(&mut self, ctx: &mut Rltk) {
//         ctx.cls();
//         ctx.print(1, 1, "Hello Rust World!");

//         // Render out our entities
//         let positions = self.ecs.read_storage::<GridPosition>();
//         let renderables = self.ecs.read_storage::<Renderable>();

//         for (pos, render) in (&positions, &renderables).join() {
//             ctx.set(pos.x, pos.y, render.fg, render.bg, render.glyph);
//         }
//     }
// }



fn main() -> GameResult {
    // use rltk::RltkBuilder;

    // let context = RltkBuilder::simple80x50()
    //     .with_title("Roguelike Tut - THAT Game")
    //     .build();
    
    // Create State with ECS world in it.
    let mut gs = State {
        ecs: World::new()
    };

    // Register components
    gs.ecs.register::<GridPosition>();
    gs.ecs.register::<Renderable>();
    gs.ecs.register::<Player>();
    gs.ecs.register::<Viewshed>();

    // Add a map to ECS resources
    // and placelace player in the center of 1st room
    let map: Map = Map::new_map_rooms_and_corridors();
    let (player_x, player_y) = map.rooms[0].center();

    // Create player
    gs.ecs
        .create_entity()
        .with(GridPosition {x: player_x, y: player_y})
        .with(Renderable {
            color: gfx::Color::new(0., 1., 0., 1.),
        })
        .with(Player {})
        .with(Viewshed {visible_tiles: Vec::new(), range: 8, dirty: true })
        .build();
    
    // Add some monsters
    for room in map.rooms.iter().skip(1) {

        let (x, y) = room.center();

        gs.ecs
        .create_entity()
            .with(GridPosition { x, y })
            .with(Renderable {
                color: gfx::Color::new(1., 0., 0., 1.),
            })
            .with(Viewshed{ visible_tiles : Vec::new(), range: 8, dirty: true})

        .build();
    }

    gs.ecs.insert(map);

    // @TODO: Screen dims to use for (80 x 50 , tile size 16) = 1280 x 800

    let cb = ggez::ContextBuilder::new("THAT GAME - super simple", "Spiros Makris");
    let (ctx, event_loop) = &mut cb.build()?;
    
    event::run(ctx, event_loop, &mut gs)
}
