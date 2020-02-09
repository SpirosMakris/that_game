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
    
    rltk::main_loop(context, gs);
}
