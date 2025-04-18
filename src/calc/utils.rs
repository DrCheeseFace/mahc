use crate::{suit::Suit, tile::Tile, tile_group::TileGroup};
use std::collections::HashMap;

pub fn get_melds_from_tile_counts(tile_counts: &HashMap<Tile, u8>) -> Vec<TileGroup> {
    let mut tiles: Vec<Tile> = Vec::new();
    for (tile, count) in tile_counts {
        for _ in 0..=*count {
            tiles.push(*tile)
        }
    }
    let trips = get_triplets(&tiles);
    let kans = get_kans(&tiles);
    let seq = get_sequences(&tiles);
    [trips, kans, seq].concat()
}

// assumes 14 tile valid number of tiles (max 4 of each tile)
pub fn get_kans(tiles: &[Tile]) -> Vec<TileGroup> {
    let mut kans: Vec<TileGroup> = Vec::new();
    for target_tile in tiles.iter() {
        let count = tiles.iter().filter(|t| **t == *target_tile).count();
        if count == 4 {
            let tilegroup: TileGroup = TileGroup::new(
                vec![*target_tile, *target_tile, *target_tile, *target_tile],
                false,
            )
            .unwrap();
            if !kans.contains(&tilegroup) {
                kans.push(tilegroup);
            }
        }
    }

    kans
}

// assumes valid max number of tiles
pub fn get_triplets(tiles: &[Tile]) -> Vec<TileGroup> {
    let mut trips: Vec<TileGroup> = Vec::new();
    for target_tile in tiles.iter() {
        let count = tiles.iter().filter(|t| **t == *target_tile).count();
        if count == 3 {
            let tilegroup: TileGroup =
                TileGroup::new(vec![*target_tile, *target_tile, *target_tile], false).unwrap();
            if !trips.contains(&tilegroup) {
                trips.push(tilegroup);
            }
        }
    }

    trips
}

// assumes valid max number of tiles
pub fn get_sequences(tiles: &[Tile]) -> Vec<TileGroup> {
    let tiles: Vec<Tile> = tiles
        .iter()
        .filter(|t| t.suit() == Suit::Manzu || t.suit() == Suit::Pinzu || t.suit() == Suit::Souzu)
        .cloned()
        .collect();
    let mut sequences: Vec<TileGroup> = Vec::new();
    for target_tile in tiles.iter() {
        if target_tile.parse_u8().unwrap() > 7 {
            continue;
        }
        let second = target_tile.get_next();
        let third = second.get_next();
        if tiles.contains(&second) && tiles.contains(&third) {
            let tile_group = TileGroup::new(vec![*target_tile, second, third], false).unwrap();
            sequences.push(tile_group);
        }
    }
    sequences
}

// assumes valid max number of tiles
pub fn get_singles(tiles: &[Tile]) -> Vec<TileGroup> {
    let mut singles: Vec<TileGroup> = Vec::new();
    for tile in tiles {
        if tiles.iter().filter(|t| **t == *tile).count() == 1 {
            let tile_group = TileGroup::new(vec![*tile], false).unwrap();
            singles.push(tile_group)
        }
    }
    singles
}

// assumes valid max number of tiles
pub fn get_pairs(tiles: &Vec<Tile>) -> Vec<TileGroup> {
    let mut pairs: Vec<TileGroup> = Vec::new();
    let mut counts = std::collections::HashMap::new();

    for tile in tiles {
        let count = counts.entry(*tile).or_insert(0);
        if *count < 4 {
            *count += 1;
        }
    }
    for (tile, count) in counts.iter() {
        for _ in 0..(count / 2) {
            let tilegroup: TileGroup = TileGroup::new(vec![*tile, *tile], false).unwrap();
            pairs.push(tilegroup)
        }
    }
    pairs
}
