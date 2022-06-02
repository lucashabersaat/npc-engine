use std::{fmt, marker::PhantomData};
use serde::Serialize;

use crate::Coord2D;

pub trait YUpDown {
	fn up() -> i32;
	fn down() -> i32;
}
pub struct YUp;
impl YUpDown for YUp {
	fn up() -> i32 { 1 }
	fn down() -> i32 { -1 }
}
pub struct YDown;
impl YUpDown for YDown {
	fn up() -> i32 { -1 }
	fn down() -> i32 { 1 }
}

#[derive(Copy, Clone, Debug, Hash, PartialEq, Eq, Serialize)]
#[serde(rename_all = "kebab-case")]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl fmt::Display for Direction {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Direction::Up => write!(f, "Up"),
            Direction::Down => write!(f, "Down"),
            Direction::Left => write!(f, "Left"),
            Direction::Right => write!(f, "Right"),
        }
    }
}

pub struct DirectionConverter<YDir: YUpDown> {
	_phantom: PhantomData<YDir>
}
impl<YDir: YUpDown> DirectionConverter<YDir> {
    pub fn apply(direction: Direction, coord: Coord2D) -> Coord2D {
        let Coord2D { x, y } = coord;
        let (x, y) = match direction {
            Direction::Up => (x, y + YDir::up()),
            Direction::Down => (x, y + YDir::down()),
            Direction::Left => (x - 1, y),
            Direction::Right => (x + 1, y),
        };
        Coord2D::new(x, y)
    }

    pub fn from(start: Coord2D, end: Coord2D) -> Direction {
		let dx = end.x - start.x;
		let dy = end.y - start.y;
        match (dx, dy) {
            (1, _) => Direction::Right,
            (-1, _) => Direction::Left,
            (_, 1) => match YDir::up() == 1 {
                true => Direction::Up,
                false => Direction::Down,
            },
            (_, -1) => match YDir::down() == -1 {
                true => Direction::Down,
                false => Direction::Up,
            },
            _ => panic!("start and end positions are not next to each others with 4-connectivity"),
        }
    }
}

pub const DIRECTIONS: [Direction; 4] = [
    Direction::Up,
    Direction::Right,
    Direction::Down,
    Direction::Left,
];
pub type DirectionConverterYUp = DirectionConverter<YUp>;
pub type DirectionConverterYDown = DirectionConverter<YDown>;