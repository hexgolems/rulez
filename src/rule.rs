use std::collections::HashMap;

#[derive(Clone, Serialize, Deserialize)]
pub struct Rule {
    pub pattern: Vec<u8>,
    pub replace: u8,
    pub mutable: bool,
}

impl Rule {
    fn is_var(p: u8) -> bool {
        return 0x40 < p && p <= 0x5a;
    }
    fn matches(p: u8, n: u8, repl: &mut HashMap<u8, u8>) -> bool {
        if p == 0x5f {
            return true;
        } // _ matches everything
        if Self::is_var(p) {
            if let Some(v) = repl.get(&p) {
                return *v == n;
            }
            repl.insert(p, n);
            return true;
        }
        return p == n;
    }
    pub fn apply(&self, neighborhood: Vec<u8>) -> Option<u8> {
        let mut repl = HashMap::new();
        if self
            .pattern
            .iter()
            .zip(neighborhood.iter())
            .all(|(p, n)| Self::matches(*p, *n, &mut repl))
        {
            return Some(*repl.get(&self.replace).unwrap_or(&self.replace));
        }
        return None;
    }
}
