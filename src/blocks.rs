use crate::{BlockType, Position};

/*
 * These functions  the start positions of the individual blocks for each unique block for
 * the game.
 *
 * The blocks will spawn in the top 4 rows on the playfield that will not be rendered in the game.
 * This is so that the blocks can slowly descend and progressively appear to the player.
 */

pub fn get_orange_start() -> [Position; 4] {
    [
        Position(3, 4),
        Position(3, 5),
        Position(2, 5),
        Position(3, 3),
    ]
}

pub fn get_blue_start() -> [Position; 4] {
    [
        Position(3, 4),
        Position(3, 3),
        Position(2, 3),
        Position(3, 5),
    ]
}

pub fn get_hero_start() -> [Position; 4] {
    [
        Position(2, 5),
        Position(3, 5),
        Position(1, 5),
        Position(0, 5),
    ]
}

pub fn get_teewee_start() -> [Position; 4] {
    [
        Position(3, 4),
        Position(3, 3),
        Position(2, 4),
        Position(3, 5),
    ]
}

pub fn get_cleveland_start() -> [Position; 4] {
    [
        Position(3, 4),
        Position(3, 5),
        Position(2, 3),
        Position(2, 4),
    ]
}

pub fn get_rhode_start() -> [Position; 4] {
    [
        Position(3, 4),
        Position(3, 3),
        Position(2, 4),
        Position(2, 5),
    ]
}

pub fn get_smash_start() -> [Position; 4] {
    [
        Position(3, 4),
        Position(3, 5),
        Position(2, 4),
        Position(2, 5),
    ]
}

pub fn get_color(block_type: &BlockType) -> String {
    match block_type {
        BlockType::None => "transparent".to_string(),
        BlockType::Hero => "#1c82d6".to_string(),
        BlockType::Teewee => "#951cd6".to_string(),
        BlockType::Smashboy => "#fbff21".to_string(),
        BlockType::BlueRicky => "#0915bd".to_string(),
        BlockType::ClevelandZ => "#e62929".to_string(),
        BlockType::OrangeRicky => "#e69a29".to_string(),
        BlockType::RhodeIslandZ => "#1fd916".to_string(),
    }
}
