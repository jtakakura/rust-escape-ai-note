use macroquad_tiled::Map;

use crate::{LAYER_DOOR, LAYER_KEYS, LAYER_PLAYER, LAYER_WALLS};

#[derive(Clone)]
pub struct LevelInfo {
    pub size: (usize, usize),
    pub key: (usize, usize),
    pub door: (usize, usize),
    pub agent: (usize, usize),
}

impl LevelInfo {
    pub fn new(map: &Map) -> Self {
        LevelInfo::parse_level(map)
    }

    fn get_one_item(w: u32, h: u32, layer: &str, map: &Map) -> Option<(usize, usize)> {
        if !map.contains_layer(layer) {
            return None;
        }

        for i in 0..w {
            for j in 0..h {
                match map.get_tile(layer, i, j) {
                    Some(_) => return Some((i as usize, j as usize)),
                    None => continue,
                }
            }
        }

        None
    }

    fn parse_level(map: &Map) -> Self {
        // Size of the map
        let w = map.layers[LAYER_WALLS].width;
        let h = map.layers[LAYER_WALLS].height;

        // Search for a door
        let door = LevelInfo::get_one_item(w, h, LAYER_DOOR, map);
        if door.is_none() {
            panic!("No door found in the map");
        }

        // Search for keys
        let key = LevelInfo::get_one_item(w, h, LAYER_KEYS, map);
        if key.is_none() {
            panic!("No key found in the map");
        }

        // Search for player spawn pos
        let player = LevelInfo::get_one_item(w, h, LAYER_PLAYER, map);
        if player.is_none() {
            panic!("No player spawn found in the map");
        }

        Self {
            size: (w as usize, h as usize),
            key: key.unwrap(),
            door: door.unwrap(),
            agent: player.unwrap(),
        }
    }
}
