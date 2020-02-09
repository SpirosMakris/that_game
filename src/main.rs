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


#[derive(Component)]
#[storage(VecStorage)]  // default is `DenseVecStorage`
struct GridPosition {
    x: i32,
    y: i32,
}

#[derive(Component)]
struct Renderable {
    glyph: u8,
    fg: RGB,
    bg: RGB,
    color: Color,
}

#[derive(Component)]
struct LeftMover {}

struct LeftWalkerSys {}

impl<'a> System<'a> for LeftWalkerSys {
    type SystemData = (ReadStorage<'a, LeftMover>,
                        WriteStorage<'a, GridPosition>);
    
    fn run(&mut self, (lefty, mut pos) : Self::SystemData) {
        for (_lefty, pos) in (&lefty, &mut pos).join() {
            pos.x -= 1;
            if pos.x < 0 { pos.x = 79; }
        }
    }
}


#[derive(Component, Debug)]
struct Player {}


fn try_move_player(delta_x: i32, delta_y: i32, ecs: &mut World) {
    let mut positions = ecs.write_storage::<GridPosition>();
    let mut players = ecs.write_storage::<Player>();

    for (_player, pos) in (&mut players, &mut positions).join() {
        pos.x = min(79, max(0, pos.x + delta_x));
        pos.y = min(49, max(0, pos.y + delta_y));
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


struct State {
    ecs: World,
}

impl State {
    fn run_systems(&mut self) {
        let mut lw = LeftWalkerSys{};
        lw.run_now(&self.ecs);
        self.ecs.maintain();
    }
}

impl event::EventHandler for State {
    fn update(&mut self, ctx: &mut Context) -> GameResult {
        player_input(self, ctx);
        self.run_systems();
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        graphics::clear(ctx, [0.1, 0.2, 0.3, 1.0].into());

        let positions = self.ecs.read_storage::<GridPosition>();
        let renderables = self.ecs.read_storage::<Renderable>();

        const GRID_TILE_SIZE: i32 = 16;

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

            graphics::draw(ctx, &circle, (na::Point2::new( (pos.x * GRID_TILE_SIZE) as f32, (pos.y * GRID_TILE_SIZE) as f32), ))?;

            
            
        }
        
        graphics::present(ctx)?;
        Ok(())
    }
}

impl GameState for State {
    fn tick(&mut self, ctx: &mut Rltk) {
        ctx.cls();
        ctx.print(1, 1, "Hello Rust World!");

        // Render out our entities
        let positions = self.ecs.read_storage::<GridPosition>();
        let renderables = self.ecs.read_storage::<Renderable>();

        for (pos, render) in (&positions, &renderables).join() {
            ctx.set(pos.x, pos.y, render.fg, render.bg, render.glyph);
        }
    }
}



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

    // Create an entity
    gs.ecs
        .create_entity()
        .with(GridPosition {x: 40, y: 25})
        .with(Renderable {
            glyph: rltk::to_cp437('@'),
            fg: RGB::named(rltk::YELLOW),
            bg: RGB::named(rltk::BLACK),
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
                glyph: rltk::to_cp437('☺'),
                fg: RGB::named(rltk::RED),
                bg: RGB::named(rltk::BLACK),
                color: graphics::Color::new(1., 0., 0., 1.),
            })
            .with(LeftMover {})
            .build();
    }
    
    // rltk::main_loop(context, gs);

    let cb = ggez::ContextBuilder::new("THAT GAME - super simple", "Spiros Makris");
    let (ctx, event_loop) = &mut cb.build()?;
    
    event::run(ctx, event_loop, &mut gs)
}
