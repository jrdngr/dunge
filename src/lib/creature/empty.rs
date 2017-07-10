use ::lib::grid::Action;
use ::lib::creature::*;

pub fn new() -> Creature {
    Creature {
        creature_type: CreatureType::Empty,
        color: [0.0, 0.0, 0.0, 1.0],
        properties: HashMap::new(),
        action: empty_action,
    }
}

#[allow(unused_variables)]
fn empty_action(neighbors: &Neighbors) -> Vec<Action> {
    vec![Action::Idle]
}