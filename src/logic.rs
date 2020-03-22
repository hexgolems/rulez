use itertools::Itertools;
use std::collections::HashMap;


#[derive(Clone, PartialEq, Eq)]
pub struct Field {
    pub w: usize,
    pub h: usize,
    pub data: Vec<Vec<u8>>,
}

impl Field {
    pub fn new(w: usize, h: usize) -> Self {
        let mut data = vec![];
        for _x in 0..w {
            data.push((0..h).map(|_| 0x20u8).collect::<Vec<u8>>());
        }
        Self { data, w, h }
    }

    pub fn coords(&self) -> itertools::Product<std::ops::Range<usize>, std::ops::Range<usize>> {
        return (0..self.w).cartesian_product(0..self.h);
    }
    pub fn set(&mut self, x: usize, y: usize, val: u8) {
        assert!(x < self.w);
        assert!(y < self.h);
        self.data[x][y] = val;
    }

    pub fn get(&self, x: isize, y: isize) -> u8 {
        if x < 0 || y < 0 {
            return 0x20;
        }
        *self
            .data
            .get(x as usize)
            .and_then(|w| w.get(y as usize))
            .unwrap_or(&0x20)
    }

    pub fn neighborhood(&self, x: usize, y: usize) -> Vec<u8> {
        let mut res = vec![];
        for yo in -1..=1 {
            for xo in -1..=1 {
                res.push(self.get((x as isize) + xo, (y as isize) + yo));
            }
        }
        return res;
    }

    pub fn to_string(&self) -> String {
        let mut res = "/".to_string();
        res += &(0..self.w).map(|_| "-").collect::<String>();
        res += "\\\n";
        for y in 0..self.h {
            res += "|";
            for x in 0..self.w {
                res += std::str::from_utf8(&[self.get(x as isize, y as isize)]).unwrap();
            }
            res += "|\n"
        }
        res += "\\";
        res += &(0..self.w).map(|_| "-").collect::<String>();
        res += "/";
        return res;
    }
}

//"x."
// A not in [ ]
// B in [   ]

// """"""""""""""""
// |              |
// |              |
// |  x           |
// |              |
// |              |
// |              |
// """"""""""""""""

// """"""""""""""""
// |              |
// |              |
// |  ..........x |
// |              |
// |              |
// |              |
// """"""""""""""""

// """""    """""   //// """""  """""
// |A A|    |   |   //// |   |  |ABC|
// |   |=x  |   |=  //// |   |  |DEF|=E
// |   |    |   |   //// |   |  |GHI|
// """""    """""   //// """""  """""

pub struct Rule {
    pub pattern: Vec<u8>,
    pub replace: u8,
    pub mutable: bool,
}

impl Rule {

    fn is_var(p: u8) -> bool{
        return 0x40 < p && p <= 0x5a;
    }
    fn matches(p: u8, n: u8, repl: &mut HashMap<u8,u8>) ->  bool {
        if Self::is_var(p) {
            if let Some(v) = repl.get(&p){
                return *v == n;
            }
            repl.insert(p,n);
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
            .all(|(p, n)| Self::matches(*p,*n, &mut repl))
        {
            return Some(*repl.get(&self.replace).unwrap_or(&self.replace));
        }
        return None;
    }
}

pub struct Automaton {
    pub rules: Vec<Rule>,
}

impl Automaton {
    pub fn new(rules: Vec<Rule>) -> Self {
        Self { rules }
    }

    pub fn step(&self, field: Field) -> Field {
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

    pub fn add_rule(&mut self, pattern: &[u8], replace: u8, mutable: bool) {
        self.rules.push(Rule {
            pattern: pattern.to_vec(),
            replace,
            mutable,
        });
    }
}

pub struct Level {
    pub start: Field,
    pub goal: Field,
    pub auto: Automaton,
}

impl Level {
    pub fn new(start: Field, goal: Field) -> Self {
        return Self {
            start,
            goal,
            auto: Automaton::new(vec![]),
        };
    }
}