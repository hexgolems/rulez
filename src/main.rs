extern crate itertools;
extern crate termion;

use std::io::{stdin, stdout, Stdout};
use termion::event::{Event, Key};
use termion::input::{MouseTerminal, TermRead};
use termion::raw::IntoRawMode;
use termion::raw::RawTerminal;
use termion::screen::AlternateScreen;
use std::thread;
use std::time;
use std::sync::{RwLock, Arc};
mod ui;
mod logic;

use logic::{Field, Level};
use ui::{UIState};

fn animation(ui: Arc<RwLock<UIState>>, out: Arc<RwLock<MouseTerminal<AlternateScreen<RawTerminal<Stdout>>>>>, lvl: Arc<RwLock<Level>>){
    loop {  
        let ten_millis = time::Duration::from_millis(500);
        thread::sleep(ten_millis); 
        ui.write().unwrap().update(&lvl.read().unwrap());
        ui.write().unwrap().draw(&mut out.write().unwrap(), &lvl.read().unwrap());
    }
}


fn main() {
    let mut start = Field::new(7, 7);
    start.set(0, 2, "x".as_bytes()[0]);
    let mut goal = Field::new(7, 7);
    for i in 0..4 {
        goal.set(i, 2, ".".as_bytes()[0]);
    }
    goal.set(4, 2, "x".as_bytes()[0]);

    let mut lvl = Level::new(start, goal);
    lvl.auto.add_rule(b"         ", " ".as_bytes()[0], true);
    lvl.auto.add_rule(b"         ", " ".as_bytes()[0], true);

    let lvl = Arc::new(RwLock::new(lvl));
    let screen = AlternateScreen::from(stdout().into_raw_mode().unwrap());
    let ui = Arc::new(RwLock::new(UIState::new(&lvl.read().unwrap())));
    let out = Arc::new(RwLock::new(MouseTerminal::from(screen)));
    
    ui.write().unwrap().draw(&mut out.write().unwrap(), &lvl.read().unwrap());
    
    let ui2 = ui.clone();
    let out2 = out.clone();
    let lvl2 = lvl.clone();
    thread::spawn(|| animation(ui2, out2, lvl2));

    for c in stdin().events() {
        let evt = c.unwrap();
        let mut uib = ui.write().unwrap();
        let mut lvlb = lvl.write().unwrap();
        match evt {
            Event::Key(Key::Char('q')) => break,
            Event::Key(Key::Up) => uib.cursor = uib.cursor.up(&lvlb),
            Event::Key(Key::Left) => uib.cursor = uib.cursor.left(&lvlb),
            Event::Key(Key::Right) => uib.cursor = uib.cursor.right(&lvlb),
            Event::Key(Key::Down) => uib.cursor = uib.cursor.down(&lvlb),
            Event::Key(Key::Backspace) => uib.reset_field(&lvlb),
            Event::Key(Key::Char(' ')) => uib.toggle(&mut lvlb),
            Event::Key(Key::Char(x)) => uib.set_char(&x, &mut lvlb),
            _ => {} ,
        }
        uib.draw(&mut out.write().unwrap(), &lvlb);
    }
}
