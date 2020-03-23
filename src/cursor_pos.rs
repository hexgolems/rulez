use crate::level::Level;

pub enum CursorPos {
    Pattern { i: isize, x: isize, y: isize },
    Replace { i: isize },
    State { last_i: isize },
}

impl CursorPos {
    pub fn in_state(&self) -> bool {
        match self {
            CursorPos::State { .. } => true,
            _ => false,
        }
    }

    pub fn up(&self, _lvl: &Level) -> Self {
        use CursorPos::*;
        match *self {
            Pattern { i, x, y } => Pattern {
                i,
                x,
                y: (y as usize).saturating_sub(1) as isize,
            },
            Replace { i } => Pattern { i: i, x: 1, y: 2 },
            State { last_i } => Replace { i: last_i },
        }
    }
    pub fn down(&self, _lvl: &Level) -> Self {
        use CursorPos::*;
        match *self {
            Pattern { i, x, y } if y < 2 => Pattern { i, x, y: y + 1 },
            Pattern { i, .. } => Replace { i },
            Replace { i } => State { last_i: i },
            State { last_i } => State { last_i },
        }
    }
    pub fn left(&self, lvl: &Level) -> Self {
        use CursorPos::*;
        let len = lvl.auto.rules.len() as isize;
        match *self {
            Pattern { i, x: 0, y } => Pattern {
                i: (len + i - 1) % len,
                x: 2,
                y,
            },
            Pattern { i, x, y } => Pattern { i, x: x - 1, y },
            Replace { i } => Replace {
                i: (len + i - 1) % len,
            },
            State { last_i } => State { last_i },
        }
    }
    pub fn right(&self, lvl: &Level) -> Self {
        use CursorPos::*;
        let len = lvl.auto.rules.len() as isize;
        match *self {
            Pattern { i, x: 2, y } => Pattern {
                i: (i + 1) % len,
                x: 0,
                y,
            },
            Pattern { i, x, y } => Pattern { i, x: x + 1, y },
            Replace { i } => Replace { i: (i + 1) % len },
            State { last_i } => State { last_i },
        }
    }
}
