use crate::{create_graph_machine, create_recipe};

use super::{
    graph_components::{GraphMachine, Recipe},
    machines::Furnace,
};

create_graph_machine!(
    GraphFurnace,
    Furnace,
    [
        create_recipe!(3.2 + iron_ore: 1 => iron_plate: 1),
        create_recipe!(3.2 + copper_ore: 1 => copper_plate: 1)
    ]
);
