use crate::default_machine;

use super::components::{Direction, Machine, Vec2};

#[derive(Debug, Clone)]
pub struct Furnace {
    id: uuid::Uuid,
    position: Vec2,
    size: Vec2,
    direction: Option<Direction>,
}
default_machine!(Furnace, Vec2::new(2, 2));

#[derive(Debug, Clone)]
pub struct Belt {
    id: uuid::Uuid,
    position: Vec2,
    size: Vec2,
    direction: Option<Direction>,
}
default_machine!(Belt, Vec2::new(1, 1));

#[derive(Debug, Clone)]
pub struct Inserter {
    id: uuid::Uuid,
    position: Vec2,
    size: Vec2,
    direction: Option<Direction>,
}
default_machine!(Inserter, Vec2::new(1, 1));
