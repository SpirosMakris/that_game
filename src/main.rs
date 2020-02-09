use specs::prelude::*;

#[macro_use]
extern crate specs_derive;

use rltk::{Console, GameState, Rltk, RGB, VirtualKeyCode};

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
}

struct State {
    ecs: World,
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



fn main() {
    use rltk::RltkBuilder;

    let context = RltkBuilder::simple80x50()
        .with_title("Roguelike Tut - THAT Game")
        .build();
    
    // Create State with ECS world in it.
    let mut gs = State {
        ecs: World::new()
    };

    // Register components
    gs.ecs.register::<GridPosition>();
    gs.ecs.register::<Renderable>();

    // Create an entity
    gs.ecs
        .create_entity()
        .with(GridPosition {x: 40, y: 25})
        .with(Renderable {
            glyph: rltk::to_cp437('@'),
            fg: RGB::named(rltk::YELLOW),
            bg: RGB::named(rltk::BLACK)
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
            })
            .build();
    }
    
    rltk::main_loop(context, gs);
}
