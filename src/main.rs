use specs::prelude::*;

#[macro_use]
extern crate specs_derive;

use rltk::{Console, GameState, Rltk, RGB, VirtualKeyCode};

use std::cmp::{min, max};

use ggez;
use ggez::event::{self, KeyCode};
use ggez::graphics::{self, Color};
use ggez::nalgebra as na;
use ggez::{Context, GameResult};
use ggez::input::keyboard;
use ggez::timer;



mod components;
pub use components::*;

#[derive(Component)]
struct LeftMover {}

// struct LeftWalkerSys {}

// impl<'a> System<'a> for LeftWalkerSys {
//     type SystemData = (ReadStorage<'a, LeftMover>,
//                         WriteStorage<'a, GridPosition>);
    
//     fn run(&mut self, (lefty, mut pos) : Self::SystemData) {
//         for (_lefty, pos) in (&lefty, &mut pos).join() {
//             pos.x -= 1;
//             if pos.x < 0 { pos.x = 79; }
//         }
//     }
// }




fn try_move_player(delta_x: i32, delta_y: i32, ecs: &mut World) {
    let mut positions = ecs.write_storage::<GridPosition>();
    let mut players = ecs.write_storage::<Player>();
    let map = ecs.fetch::<Vec<TileType>>();

    for (_player, pos) in (&mut players, &mut positions).join() {
        let destination_idx = xy_idx(pos.x + delta_x, pos.y + delta_y);
        if map[destination_idx] != TileType::Wall {
            pos.x = min(79, max(0, pos.x + delta_x));
            pos.y = min(49, max(0, pos.y + delta_y));
        }
    }
}

fn player_input(gs: &mut State, ctx: &Context) {
    let pressed_keys = keyboard::pressed_keys(ctx);
    if pressed_keys.is_empty() { return };
    
    if keyboard::is_key_pressed(ctx, KeyCode::Left) {
        try_move_player(-1, 0, &mut gs.ecs);
    }

    if keyboard::is_key_pressed(ctx, KeyCode::Right) {
        try_move_player(1, 0, &mut gs.ecs);
    }

    if keyboard::is_key_pressed(ctx, KeyCode::Up) {
        try_move_player(0, -1, &mut gs.ecs);
    }

    if keyboard::is_key_pressed(ctx, KeyCode::Down) {
        try_move_player(0, 1 , &mut gs.ecs);
    }
}

// MAP
const GRID_TILE_SIZE: i32 = 16;

#[derive(PartialEq, Copy, Clone)]
enum TileType {
    Wall,
    Floor,
}

pub fn xy_idx(x: i32, y: i32) -> usize {
    (y as usize * 80) + x as usize
}

fn new_map() -> Vec<TileType> {
    let mut map = vec![TileType::Floor; 80 * 50];

    // Make the boundaries walls
    for x in 0..80 {
        map[xy_idx(x, 0)]  = TileType::Wall;
        map[xy_idx(x, 49)] = TileType::Wall;
    }

    for y in 0..50 {
        map[xy_idx(0, y)] = TileType::Wall;
        map[xy_idx(79, y)] = TileType::Wall;
    }

    // Now we'll randomly splat a bunch of walls.
    // It won't be pretty, but it's a decent illustration.
    // First obtain the thread-local RNG. @RLTK
    let mut rng = rltk::RandomNumberGenerator::new();

    // Only 1/10th of the map will be walls
    for _i in 0..400 {
        let x = rng.roll_dice(1, 79);
        let y = rng.roll_dice(1, 49);

        let idx = xy_idx(x, y);
        // Center is always Floor -> Player's starting point
        if idx != xy_idx(40, 25) {
            map[idx] = TileType::Wall;
        }
    }

    map
}

fn draw_map(map: &[TileType], ctx: &mut Context) -> GameResult {
    use ggez::graphics::{MeshBuilder, DrawMode};

    let mut x = 0;
    let mut y = 0;

    let mut mesh = &mut MeshBuilder::new();

    for tile in map.iter() {
        // Render a tile depending upon it's tiletype
        let color = match tile {
            TileType::Floor => {
                Color::new(0.0, 1.0, 0.0, 0.5)
            },
            TileType::Wall => {
                Color::new(1.0, 0.0, 0.0, 1.0)
            }
        };
        let rect = graphics::Rect::new_i32(x * GRID_TILE_SIZE, y * GRID_TILE_SIZE, GRID_TILE_SIZE, GRID_TILE_SIZE);
        mesh = mesh.rectangle(DrawMode::fill(), rect, color);
        // let r1_mesh = graphics::Mesh::new_rectangle(ctx, graphics::DrawMode::fill(), rect, color)?;
        // graphics::draw(ctx, &r1_mesh, graphics::DrawParam::default())?;

        // Move the coordinates
        x += 1;
        if x > 79 {
            x = 0;
            y += 1;
        }
    }
    
    let mesh = mesh.build(ctx)?;

    graphics::draw(ctx, &mesh, graphics::DrawParam::default())?;
    Ok(())
}

// GAME STATE

struct State {
    ecs: World,
}

impl State {
    fn run_systems(&mut self) {
        // let mut lw = LeftWalkerSys{};
        // lw.run_now(&self.ecs);
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
        graphics::clear(ctx, [0.1, 0.2, 0.3, 1.0].into());

        // Render our map
        let map = self.ecs.fetch::<Vec<TileType>>();
        draw_map(&map, ctx)?;

        let positions = self.ecs.read_storage::<GridPosition>();
        let renderables = self.ecs.read_storage::<Renderable>();

        for (pos, rnd) in (&positions, &renderables).join() {

            let circle = graphics::Mesh::new_circle(
                ctx, 
                graphics::DrawMode::fill(),
                na::Point2::new(0.0, 0.0), 
                10.0, 
                2.0,
                // graphics::WHITE,
                rnd.color
            )?;

            graphics::draw(ctx, &circle, (na::Point2::new( (pos.x * GRID_TILE_SIZE) as f32 + (GRID_TILE_SIZE / 2) as f32, (pos.y * GRID_TILE_SIZE) as f32 + (GRID_TILE_SIZE / 2) as f32), ))?;
        }

        
        graphics::present(ctx)?;
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
    gs.ecs.register::<LeftMover>();
    gs.ecs.register::<Player>();

    // Add a map to ECS resources
    gs.ecs.insert(new_map());

    // Create an entity
    gs.ecs
        .create_entity()
        .with(GridPosition {x: 40, y: 25})
        .with(Renderable {
            color: graphics::Color::new(0., 1., 0., 1.),
        })
        .with(Player {})
        .build();
    
    // Add a bunch more entities
    for i in 0..10 {
        gs.ecs
            .create_entity()
            .with(GridPosition { x: i * 7, y: 20 })
            .with(Renderable {
                color: graphics::Color::new(1., 0., 0., 1.),
            })
            .with(LeftMover {})
            .build();
    }
    
    // rltk::main_loop(context, gs);

    // @TODO: Screen dims to use for (80 x 50 , tile size 16) = 1280 x 800

    let cb = ggez::ContextBuilder::new("THAT GAME - super simple", "Spiros Makris");
    let (ctx, event_loop) = &mut cb.build()?;
    
    event::run(ctx, event_loop, &mut gs)
}
