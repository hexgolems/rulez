use crate::automaton::Automaton;
use crate::field::Field;

#[derive(Clone, Serialize, Deserialize)]
pub struct Level {
    pub start: Field,
    pub goal: Field,
    pub auto: Automaton,
    pub id: usize,
}

impl Level {
    //pub fn new(start: Field, goal: Field) -> Self {
    //    return Self {
    //        start,
    //        goal,
    //        auto: Automaton::new(vec![]),
    //    };
    //}
}
