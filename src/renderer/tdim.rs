use crate::loader::pkrc::{PkRcTilemap, PkRcParser, TileType};
use macroquad::prelude::*;

pub struct TDimRenderer {
    tilemap: PkRcTilemap,
    width_divisor: f32,
    height_divisor: f32,
    player_position: (f32, f32),
    player_rotation: (f32, f32),
    player_spawned: bool,
}

impl TDimRenderer {
    pub fn new() -> Self {

        let tilemap = PkRcParser::parse("assets/maps/testmap/floor1.pkrc");
        Self {
            width_divisor:  screen_width() / PkRcParser::longest_tile_line_len(tilemap.clone()) as f32,
            height_divisor: screen_height() / tilemap.height as f32,
            tilemap,
            player_position: (0.0, 0.0),
            player_rotation: (0.0, 0.0),
            player_spawned: false,
        }
    }

    pub fn draw_tile_xy(&mut self, ttype: TileType, x: usize, y: usize) {
        match ttype {
            TileType::Air => {},
            TileType::Error => {},
            TileType::PlayerSpawn => {
                if !self.player_spawned {
                    self.player_position = (x as f32, y as f32);
                    self.player_spawned = true;
                }
            },
            TileType::Wall => {
                draw_rectangle(x as f32 * self.width_divisor as f32, y as f32 * self.height_divisor, self.width_divisor, self.height_divisor, RED);
            }
        }
    }

    pub fn draw_player_xy(&self, x: f32, y: f32) {
        draw_rectangle(x as f32 * self.width_divisor as f32, y as f32 * self.height_divisor, 10.0, 15.0, YELLOW);
    }

    pub fn update(&mut self) {
        if is_key_down(KeyCode::W) {
            self.player_position.1 -= get_frame_time() * 2.0;
        }
    }

    pub fn render(&mut self) {
        for y in 0..self.tilemap.clone().height {
            for x in 0..self.tilemap.tilemap[y].len() {
                self.draw_tile_xy(self.tilemap.tilemap[y][x].clone(), x, y)
            }
        }
        self.draw_player_xy(self.player_position.0, self.player_position.1)
    }
}