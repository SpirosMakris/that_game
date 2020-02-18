use specs::prelude::*;

#[derive(Component)]
#[storage(VecStorage)]  // default is `DenseVecStorage`
pub struct GridPosition {
  pub  x: i32,
  pub  y: i32,
}

use ggez::graphics::Color;
#[derive(Component)]
pub struct Renderable {
  pub  color: Color,
}


#[derive(Component, Debug)]
pub struct Player {}

#[derive(Component)]
pub struct Viewshed {
  pub visible_tiles: Vec<rltk::Point>,  // @TODO: See if this needs replacing?
  pub range: i32,
  pub dirty: bool,
}

#[derive(Component, Debug)]
pub struct Monster {}

#[derive(Component, Debug)]
pub struct Name {
  pub name: String,
}

#[derive(Component, Debug)]
pub struct BlocksTile {}