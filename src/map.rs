use ggez::graphics as gfx;
use ggez::{Context, GameResult};
use std::cmp::{max, min};
extern crate specs;
use specs::prelude::*;

use super::Rect32;

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
    pub revealed_tiles: Vec<bool>,
    pub visible_tiles: Vec<bool>,
    pub blocked: Vec<bool>,
    pub tile_content: Vec<Vec<Entity>>,
}

impl Map {
    pub fn xy_idx(&self, x: i32, y: i32) -> usize {
        (y as usize * self.width as usize) + x as usize
    }

    fn is_exit_valid(&self, x: i32, y: i32) -> bool {
        if x < 1 || x > self.width - 1 || y < 1 || y > self.height - 1 {
            return false;
        }

        let idx = self.xy_idx(x, y);
        !self.blocked[idx]
    }

    pub fn populate_blocked(&mut self) {
        for (i, tile) in self.tiles.iter_mut().enumerate() {
            self.blocked[i] = *tile == TileType::Wall;
        }
    }

    pub fn clear_content_index(&mut self) {
        for content in self.tile_content.iter_mut() {
            content.clear();
        }
    }

    fn apply_room_to_map(&mut self, room: &Rect32) {
        for y in room.y1 + 1..=room.y2 {
            for x in room.x1 + 1..=room.x2 {
                let idx = self.xy_idx(x, y);
                self.tiles[idx] = TileType::Floor;
            }
        }
    }

    fn apply_horizontal_tunnel(&mut self, x1: i32, x2: i32, y: i32) {
        for x in min(x1, x2)..=max(x1, x2) {
            let idx = self.xy_idx(x, y);
            if idx > 0 && idx < self.width as usize * self.height as usize {
                self.tiles[idx] = TileType::Floor;
            }
        }
    }

    fn apply_vertical_tunnel(&mut self, y1: i32, y2: i32, x: i32) {
        for y in min(y1, y2)..=max(y1, y2) {
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
            revealed_tiles: vec![false; 80 * 50],
            visible_tiles: vec![false; 80 * 50],
            blocked: vec![false; 80 * 50],
            tile_content: vec![Vec::new(); 80 * 50],
        };

        // @TODO: Remove. Just a test to see if we can actually render an empty map
        map.revealed_tiles[0] = true;

        const MAX_ROOMS: i32 = 30;
        const MIN_SIZE: i32 = 6;
        const MAX_SIZE: i32 = 10;

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
                if new_room.intersect(other_room) {
                    ok = false
                };
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

impl rltk::Algorithm2D for Map {
    fn dimensions(&self) -> rltk::Point {
        rltk::Point::new(self.width, self.height)
    }
}

impl rltk::BaseMap for Map {
    fn is_opaque(&self, idx: usize) -> bool {
        self.tiles[idx] == TileType::Wall
    }

    fn get_available_exits(&self, idx: usize) -> Vec<(usize, f32)> {
        let mut exits: Vec<(usize, f32)> = Vec::new();
        let x = idx as i32 % self.width;
        let y = idx as i32 / self.width;
        let w = self.width as usize;

        // Cardinal directions
        if self.is_exit_valid(x - 1, y) {
            exits.push((idx - 1, 1.0))
        };
        if self.is_exit_valid(x + 1, y) {
            exits.push((idx + 1, 1.0))
        };
        if self.is_exit_valid(x, y - 1) {
            exits.push((idx - w, 1.0))
        };
        if self.is_exit_valid(x, y + 1) {
            exits.push((idx + w, 1.0))
        };

        // Diagonals
        if self.is_exit_valid(x - 1, y - 1) {
            exits.push(((idx - w) - 1, 1.45));
        }
        if self.is_exit_valid(x + 1, y - 1) {
            exits.push(((idx - w) + 1, 1.45));
        }
        if self.is_exit_valid(x - 1, y + 1) {
            exits.push(((idx + w) - 1, 1.45));
        }
        if self.is_exit_valid(x + 1, y + 1) {
            exits.push(((idx + w) + 1, 1.45));
        }

        exits
    }
}

/// Only draw tiles in the player's viewshed, using Map's revelead tile cache
pub fn draw_map(ecs: &World, ctx: &mut Context) -> GameResult {
    let map = ecs.fetch::<Map>();

    let mut y = 0;
    let mut x = 0;

    let mut map_mesh = &mut gfx::MeshBuilder::new();

    for (idx, tile) in map.tiles.iter().enumerate() {
        // Render a tile depending upon the tile type
        if map.revealed_tiles[idx] {
            let mut color = match tile {
                TileType::Floor => gfx::Color::new(0.0, 1.0, 0.0, 0.5),
                TileType::Wall => gfx::Color::new(1.0, 0.0, 0.0, 1.0),
            };

            // Grayscale the color if it's not currently visible
            if !map.visible_tiles[idx] {
                color = gfx::Color::new(0.5, 0.5, 0.5, 1.0)
            }

            let rect = gfx::Rect::new_i32(
                x * GRID_TILE_SIZE,
                y * GRID_TILE_SIZE,
                GRID_TILE_SIZE,
                GRID_TILE_SIZE,
            );
            map_mesh = map_mesh.rectangle(gfx::DrawMode::fill(), rect, color);
        }

        // Move the coordinates
        x += 1;
        if x > 79 {
            x = 0;
            y += 1;
        }
    }

    let map_mesh = map_mesh.build(ctx)?;
    gfx::draw(ctx, &map_mesh, gfx::DrawParam::default())?;

    Ok(())
}

/// Only draws tiles in the player's viewshed
pub fn __draw_map(ecs: &World, ctx: &mut Context) -> GameResult {
    use super::components::{Player, Viewshed};

    let mut viewsheds = ecs.write_storage::<Viewshed>();
    let mut players = ecs.write_storage::<Player>();
    let map = ecs.fetch::<Map>();

    for (_player, viewshed) in (&mut players, &mut viewsheds).join() {
        let mut y = 0;
        let mut x = 0;

        let mut map_mesh = &mut gfx::MeshBuilder::new();

        for tile in map.tiles.iter() {
            // Render a tile depending on the tile type
            let pt = rltk::Point::new(x, y);
            if viewshed.visible_tiles.contains(&pt) {
                let color = match tile {
                    TileType::Floor => gfx::Color::new(0.0, 1.0, 0.0, 0.5),
                    TileType::Wall => gfx::Color::new(1.0, 0.0, 0.0, 1.0),
                };

                let rect = gfx::Rect::new_i32(
                    x * GRID_TILE_SIZE,
                    y * GRID_TILE_SIZE,
                    GRID_TILE_SIZE,
                    GRID_TILE_SIZE,
                );
                map_mesh = map_mesh.rectangle(gfx::DrawMode::fill(), rect, color);
            }

            // Move the coordinates
            x += 1;
            if x > 79 {
                x = 0;
                y += 1;
            }
        }

        let map_mesh = map_mesh.build(ctx)?;
        gfx::draw(ctx, &map_mesh, gfx::DrawParam::default())?;
    }

    Ok(())
}

pub fn ___draw_map(map: &[TileType], ctx: &mut Context) -> GameResult {
    let mut x = 0;
    let mut y = 0;

    let mut mesh = &mut gfx::MeshBuilder::new();

    for tile in map.iter() {
        // Render a tile depending upon it's tiletype
        let color = match tile {
            TileType::Floor => gfx::Color::new(0.0, 1.0, 0.0, 0.5),
            TileType::Wall => gfx::Color::new(1.0, 0.0, 0.0, 1.0),
        };
        let rect = gfx::Rect::new_i32(
            x * GRID_TILE_SIZE,
            y * GRID_TILE_SIZE,
            GRID_TILE_SIZE,
            GRID_TILE_SIZE,
        );
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
