use ggez::graphics as gfx;
// use ggez::graphics::{MeshBuilder, DrawMode, Color};
use ggez::{Context, GameResult};


pub const GRID_TILE_SIZE: i32 = 16;

#[derive(PartialEq, Copy, Clone)]
pub enum TileType {
    Wall,
    Floor,
}

pub fn xy_idx(x: i32, y: i32) -> usize {
  (y as usize * 80) + x as usize
}

/// Makes a map with solid boundaries and 400 randomly placed walls. No guarantees that it won't
/// look awful.
pub fn new_map_test() -> Vec<TileType> {
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

pub fn draw_map(map: &[TileType], ctx: &mut Context) -> GameResult {

  let mut x = 0;
  let mut y = 0;

  let mut mesh = &mut gfx::MeshBuilder::new();

  for tile in map.iter() {
      // Render a tile depending upon it's tiletype
      let color = match tile {
          TileType::Floor => {
              gfx::Color::new(0.0, 1.0, 0.0, 0.5)
          },
          TileType::Wall => {
              gfx::Color::new(1.0, 0.0, 0.0, 1.0)
          }
      };
      let rect = gfx::Rect::new_i32(x * GRID_TILE_SIZE, y * GRID_TILE_SIZE, GRID_TILE_SIZE, GRID_TILE_SIZE);
      mesh = mesh.rectangle(gfx::DrawMode::fill(), rect, color);
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

  gfx::draw(ctx, &mesh, gfx::DrawParam::default())?;
  Ok(())
}