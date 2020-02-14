use ggez::graphics as gfx;
use ggez::{Context, GameResult};
use std::cmp::{max, min};

use super::{Rect32};

pub const GRID_TILE_SIZE: i32 = 8;

#[derive(PartialEq, Copy, Clone)]
pub enum TileType {
    Wall,
    Floor,
}

pub struct Map {
    pub tiles: Vec<TileType>,
    pub rooms: Vec<Rect32>,
    pub width: i32,
    pub height: i32,
}

impl Map {
    
    pub fn xy_idx(&self, x: i32, y: i32) -> usize {
      (y as usize * self.width as usize) + x as usize
    }

    fn apply_room_to_map(&mut self, room: &Rect32) {
        for y in room.y1 + 1 ..= room.y2 {
            for x in room.x1 + 1 ..= room.x2 {
                let idx = self.xy_idx(x, y);
                self.tiles[idx] = TileType::Floor;
            }
        }
    }


    fn apply_horizontal_tunnel(&mut self, x1:i32, x2:i32, y:i32) {
        for x in min(x1,x2) ..= max(x1,x2) {
            let idx = self.xy_idx(x, y);
            if idx > 0 && idx < self.width as usize * self.height as usize {
                self.tiles[idx] = TileType::Floor;
            }
        }
    }


    fn apply_vertical_tunnel(&mut self, y1:i32, y2:i32, x:i32) {
        for y in min(y1,y2) ..= max(y1,y2) {
            let idx = self.xy_idx(x, y);
            if idx > 0 && idx < self.width as usize * self.height as usize {
                self.tiles[idx] = TileType::Floor;
            }
        }
    }


    /// Makes a new map using the algorithm from http://rogueliketutorials.com/tutorials/tcod/part-3/
    /// This gives a handful of random rooms and corridors joining them together.
    pub fn new_map_rooms_and_corridors() -> Map {

        let mut map = Map {
            tiles: vec![TileType::Wall; 80 * 50],
            rooms: Vec::new(),
            width: 80,
            height: 50,
        };

        const MAX_ROOMS : i32 = 30;
        const MIN_SIZE : i32 = 6;
        const MAX_SIZE : i32 = 10;

        let mut rng = rltk::RandomNumberGenerator::new();

        for _ in 0..MAX_ROOMS {
            // Create a new room candidate
            let w = rng.range(MIN_SIZE, MAX_SIZE);
            let h = rng.range(MIN_SIZE, MAX_SIZE);
            let x = rng.roll_dice(1, 80 - w - 1) - 1;
            let y = rng.roll_dice(1, 50 - h - 1) - 1;

            let new_room = Rect32::new(x, y, w, h);

            // Check for intersections with all other rooms
            let mut ok = true;

            for other_room in map.rooms.iter() {
                if new_room.intersect(other_room) { ok = false};
            }

            // If valid apply room
            if ok {
                map.apply_room_to_map(&new_room);

                // Connect with corridor
                if !map.rooms.is_empty() {
                    let (new_x, new_y) = new_room.center();
                    let (prev_x, prev_y) = map.rooms[map.rooms.len() - 1].center();

                    if rng.range(0, 2) == 1 {
                        map.apply_horizontal_tunnel(prev_x, new_x, prev_y);
                        map.apply_vertical_tunnel(prev_y, new_y, new_x);
                    } else {
                        map.apply_vertical_tunnel(prev_y, new_y, prev_x);
                        map.apply_horizontal_tunnel(prev_x, new_x, new_y);
                    }
                }

                map.rooms.push(new_room);
            
            }
        }

        map
    }
}


// /// Makes a map with solid boundaries and 400 randomly placed walls. No guarantees that it won't
// /// look awful.
// pub fn new_map_test() -> Vec<TileType> {
//   let mut map = vec![TileType::Floor; 80 * 50];

//   // Make the boundaries walls
//   for x in 0..80 {
//       map[xy_idx(x, 0)]  = TileType::Wall;
//       map[xy_idx(x, 49)] = TileType::Wall;
//   }

//   for y in 0..50 {
//       map[xy_idx(0, y)] = TileType::Wall;
//       map[xy_idx(79, y)] = TileType::Wall;
//   }

//   // Now we'll randomly splat a bunch of walls.
//   // It won't be pretty, but it's a decent illustration.
//   // First obtain the thread-local RNG. @RLTK
//   let mut rng = rltk::RandomNumberGenerator::new();

//   // Only 1/10th of the map will be walls
//   for _i in 0..400 {
//       let x = rng.roll_dice(1, 79);
//       let y = rng.roll_dice(1, 49);

//       let idx = xy_idx(x, y);
//       // Center is always Floor -> Player's starting point
//       if idx != xy_idx(40, 25) {
//           map[idx] = TileType::Wall;
//       }
//   }

//   map
// }







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