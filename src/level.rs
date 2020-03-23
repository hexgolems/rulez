use crate::field::{Field};
use crate::automaton::{Automaton};

#[derive(Clone,Serialize, Deserialize)]
pub struct Level {
    pub start: Field,
    pub goal: Field,
    pub auto: Automaton
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