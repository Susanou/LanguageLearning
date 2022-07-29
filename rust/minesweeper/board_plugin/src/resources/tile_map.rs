use crate::resources::tile::Tile;
use crate::components::Coordinates;

use std::ops::{Deref, DerefMut};
use rand::{thread_rng, Rng};


/// Delta coordinates for all 8 square neighbors
const SQUARE_COORDINATES: [(i8, i8); 8] = [
    // Bottom left
    (-1, -1),
    // Bottom
    (0, -1),
    // Bottom right
    (1, -1),
    // Left
    (-1, 0),
    // Right
    (1, 0),
    // Top Left
    (-1, 1),
    // Top
    (0, 1),
    // Top right
    (1, 1),
];


/// Base Tile Map
#[derive(Debug, Clone)]
pub struct TileMap {
    bomb_count: u16,
    height: u16,
    width: u16,
    map: Vec<Vec<Tile>>,
}

impl TileMap {

    //Generate an emtpy tile map
    pub fn empty(width: u16, height: u16) -> Self {
        let map = (0..width)
            .into_iter()
            .map(|_| (0..height)
                        .into_iter()
                        .map(|_| Tile::Empty)
                        .collect())
            .collect();
        
            Self {
                bomb_count: 0,
                height,
                width,
                map,
            }
    }

    pub fn safe_square_at(&self, coordinates: Coordinates) -> impl Iterator<Item = Coordinates> {
        SQUARE_COORDINATES
            .iter()
            .copied()
            .map(move |tuple| coordinates + tuple)
    }

    pub fn is_bomb_at(&self, coord: Coordinates) -> bool {
        if coord.x >= self.width || coord.y >= self.height {
            return false;
        }

        self.map[coord.x as usize][coord.y as usize].is_bomb()
    }

    pub fn bomb_count_at(&self, coord: Coordinates) -> u8 {
        if self.is_bomb_at(coord)
        {
            return 0;
        }

        let res = self
                    .safe_square_at(coord)
                    .filter(|b| self.is_bomb_at(*b))
                    .count();
        
        res as u8
    }

    /// Place bombs and set neighbors
    pub fn set_bombs(&mut self, bomb_count: u16) {
        self.bomb_count = bomb_count;
        let mut remaining_bombs = bomb_count;
        let mut rng = thread_rng();
        // Place bombs
        while remaining_bombs > 0 {
            let (x, y) = (
                rng.gen_range(0..self.width) as usize,
                rng.gen_range(0..self.height) as usize,
            );
            if let Tile::Empty = self[x][y] {
                self[x][y] = Tile::Bomb;
                remaining_bombs -= 1;
            }
        }
        // Place bomb neighbors
        for x in 0..self.width {
            for y in 0..self.height {
                let coords = Coordinates { x, y };
                if self.is_bomb_at(coords) {
                    continue;
                }
                let num = self.bomb_count_at(coords);
                if num == 0 {
                    continue;
                }
                let tile = &mut self[x as usize][y as usize];
                *tile = Tile::BombNeighbor(num);
            }
        }
    }

    #[cfg(feature = "debug")]
    pub fn console_output(&self) -> String {
        let mut buffer = format!(
            "Map ({}, {}) with {} bombs:\n",
            self.width, self.height, self.bomb_count
        );

        let line: String = (0..=(self.width+1))
                            .into_iter()
                            .map(|_| '-')
                            .collect();
        
        buffer = format!("{}{}\n", buffer, line);

        for line in self.iter().rev() {
            buffer = format!("{}|", buffer);
            for tile in line.iter() {
                buffer = format!("{}{}", buffer, tile.console_output());
            }
            buffer = format!("{}|\n", buffer);
        }
        format!("{}{}", buffer, line)

    }

    // Getter functions
    pub fn width(&self) -> u16 {
        self.width
    }

    pub fn height(&self) -> u16 {
        self.height
    }

    pub fn bomb_count(&self) -> u16 {
        self.bomb_count
    }
}

impl Deref for TileMap {
    type Target = Vec<Vec<Tile>>;

    fn deref(&self) -> &Self::Target {
        &self.map
    }
}

impl DerefMut for TileMap {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.map
    }
}