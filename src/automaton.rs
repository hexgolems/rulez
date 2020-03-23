use crate::field::Field;
use crate::rule::Rule;

#[derive(Clone, Serialize, Deserialize)]
pub struct Automaton {
    pub rules: Vec<Rule>,
}

impl Automaton {
    //pub fn new(rules: Vec<Rule>) -> Self {
    //    Self { rules }
    //}

    pub fn step(&self, field: &Field) -> Field {
        let mut next_field = field.clone();
        for (x, y) in field.coords() {
            for r in self.rules.iter() {
                if let Some(res) = r.apply(field.neighborhood(x, y)) {
                    next_field.set(x, y, res);
                    break;
                }
            }
        }
        return next_field;
    }

    //pub fn add_rule(&mut self, pattern: &[u8], replace: u8, mutable: bool) {
    //    self.rules.push(Rule {
    //        pattern: pattern.to_vec(),
    //        replace,
    //        mutable,
    //    });
    //}
}
