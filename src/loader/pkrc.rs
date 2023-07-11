#[derive(Clone)]
pub enum TileType {
    Wall,
    Air,
    PlayerSpawn,
    Error,
}

pub struct PkRcParser;

#[derive(Clone)]
pub struct PkRcTilemap {
    pub height: usize,
    pub tilemap: Vec<Vec<TileType>>,
}

impl PkRcParser {
    pub fn parse(pkrc_path: &str) -> PkRcTilemap {
        let mut tilemap: Vec<Vec<TileType>> = vec![];

        let mut file_contents = std::fs::read_to_string(pkrc_path).expect(format!("Failed to read .pkrc file at: {}", pkrc_path).as_str());
        file_contents = file_contents.replace("\r", "");
    
        let file_lines = file_contents.split("\n");
        
        let mut index: usize = 0;

        for line in file_lines {
            let mut tilemap_line: Vec<TileType> = vec![];
            for tile in line.split(" ") {
                let t: TileType = match tile {
                    "1" => TileType::Wall,
                    "0" => TileType::Error,
                    "P" => TileType::PlayerSpawn,
                    _ => TileType::Air,
                };
                tilemap_line.push(t);
            }
            tilemap.push(tilemap_line);
            index += 1;
        }

        return PkRcTilemap { height: index, tilemap };
    }

    pub fn longest_tile_line_len(tilemap: PkRcTilemap) -> usize {
        let mut largest = 0;
        for tile_line in tilemap.tilemap {
            largest = if tile_line.len() > largest {
                tile_line.len()
            } else {
                largest
            }
        }
        return largest - 1;
    }
}