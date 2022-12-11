use super::components::{Direction, Machine, Vec2};

#[derive(Debug, Clone)]
pub struct Item {
    game_id: String,
    amount: Option<u32>,
}

impl Item {
    pub fn new(game_id: String, amount: Option<u32>) -> Self {
        Self { game_id, amount }
    }
}

#[derive(Debug, Clone)]
pub struct Recipe {
    input: Vec<Item>,
    output: Vec<Item>,
    time: f32,
}

impl Recipe {
    pub fn new(input: Vec<Item>, output: Vec<Item>, time: f32) -> Self {
        Self {
            input,
            output,
            time,
        }
    }
}

pub trait GraphMachine: Sync {
    fn id(&self) -> uuid::Uuid;
    fn to_normal_machine(&self, pos: Vec2, dir: Option<Direction>) -> Box<dyn Machine>;
    fn recipes() -> Vec<Recipe>;
}

#[macro_export]
macro_rules! create_graph_machine {
    ($machine:ident, $real:ty, [$($recipe:expr),*]) => {
        #[derive(Debug, Clone)]
        pub struct $machine {
            id: uuid::Uuid,
        }
        impl $machine {
            pub fn new() -> Self {
                Self {
                    id: uuid::Uuid::new_v4(),
                }
            }
        }
        impl GraphMachine for $machine {
            fn id(&self) -> uuid::Uuid {
                self.id
            }
            fn to_normal_machine(
                &self,
                pos: crate::factorio::components::Vec2,
                dir: Option<crate::factorio::components::Direction>,
            ) -> Box<dyn crate::factorio::components::Machine> {
                <$real>::new_boxed(pos, dir)
            }
            fn recipes() -> Vec<Recipe> {
                vec![$($recipe),*]
            }
        }
    };
}

#[macro_export]
macro_rules! create_recipe {
    ($time:literal + $($input:ident: $input_amount:expr),* => $($output:ident: $output_amount:expr),*) => {
        Recipe::new(
            vec![$(crate::factorio::graph_components::Item::new(
                stringify!($input).to_string().replace("_", "-"),
                Some($input_amount)
            )),*],
            vec![$(crate::factorio::graph_components::Item::new(
                stringify!($output).to_string().replace("_", "-"),
                Some($output_amount),
        )),*],
            $time
        )
    };
}
