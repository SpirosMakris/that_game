use specs::prelude::*;

#[macro_use]
extern crate specs_derive;

use rltk::{Console, GameState, Rltk, RGB, VirtualKeyCode};

use ggez;
use ggez::event;
use ggez::graphics::{self, Color};
use ggez::nalgebra as na;
use ggez::{Context, GameResult};

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
    fn update(&mut self, _ctx: &mut Context) -> GameResult {
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
