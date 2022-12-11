use factorio::{
    components::{Direction, Vec2},
    machines::{Belt, Furnace, Inserter},
};

mod factorio;

fn main() {
    // Example
    let mut blueprint = factorio::components::Blueprint::new();
    blueprint.add_machines(vec![
        Belt::new_boxed(Vec2::new(0, 1), Some(Direction::East)),
        Belt::new_boxed(Vec2::new(1, 1), Some(Direction::East)),
        Inserter::new_boxed(Vec2::new(2, 1), Some(Direction::East)),
        Furnace::new_boxed(Vec2::new(3, 0), None),
        Inserter::new_boxed(Vec2::new(5, 1), Some(Direction::East)),
        Belt::new_boxed(Vec2::new(6, 1), Some(Direction::East)),
        Belt::new_boxed(Vec2::new(7, 1), Some(Direction::East)),
    ]);

    blueprint.draw();
}
