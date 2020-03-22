extern crate itertools;
extern crate termion;

use std::io::{stdin, stdout, Write};
use termion::event::{Event, Key, MouseEvent};
use termion::input::{MouseTerminal, TermRead};
use termion::raw::IntoRawMode;

mod ui;
mod logic;

use logic::{Field, Level};
use ui::{UIState, CursorPos};

fn main() {
    let mut start = Field::new(5, 5);
    start.set(0, 2, "x".as_bytes()[0]);
    let mut goal = Field::new(5, 5);
    for i in 0..4 {
        goal.set(i, 2, ".".as_bytes()[0]);
    }
    goal.set(4, 2, "x".as_bytes()[0]);

    let mut lvl = Level::new(start, goal);
    lvl.auto.add_rule(b"   Bx    ", ".".as_bytes()[0], true);
    lvl.auto.add_rule(b"   x     ", "x".as_bytes()[0], true);

    println!("START:\n{}", lvl.start.to_string());
    println!("GOAL:\n{}", lvl.goal.to_string());

    let mut fld = lvl.start.clone(); 
    for i in 0..5 {
        fld = lvl.auto.step(fld);
        println!("STEP {}:\n{}", i,fld.to_string());
        if fld == lvl.goal{
            println!("VICTORY");
            break;
        }
    }
    println!("GOAL:\n{}", lvl.goal.to_string());

    let mut ui = UIState{cursor: CursorPos::Pattern{i: 0, x:0, y:0}};
    let mut out = MouseTerminal::from(stdout().into_raw_mode().unwrap());
    
    ui.draw(&mut out, &lvl);
    
    for c in stdin().events() {
        let evt = c.unwrap();
        match evt {
            Event::Key(Key::Char('q')) => break,
            Event::Key(Key::Up) => ui.cursor = ui.cursor.up(&lvl),
            Event::Key(Key::Left) => ui.cursor = ui.cursor.left(&lvl),
            Event::Key(Key::Right) => ui.cursor = ui.cursor.right(&lvl),
            Event::Key(Key::Down) => ui.cursor = ui.cursor.down(&lvl),
            Event::Mouse(_me) => {},
            _ => {}
        }
        ui.draw(&mut out, &lvl);
    }
    //let stdin = stdin();
    //let mut stdout = MouseTerminal::from(stdout().into_raw_mode().unwrap());
    //
    //write!(stdout, "{}{}q to exit. Click, click, click!", termion::clear::All, termion::cursor::Goto(1, 1)).unwrap();
    //stdout.flush().unwrap();
    //
    //for c in stdin.events() {
    //    let evt = c.unwrap();
    //    match evt {
    //        Event::Key(Key::Char('q')) => break,
    //        Event::Mouse(me) => {
    //            match me {
    //                MouseEvent::Press(_, x, y) => {
    //                    write!(stdout, "{}x", termion::cursor::Goto(x, y)).unwrap();
    //                },
    //                _ => (),
    //            }
    //        }
    //        _ => {}
    //    }
    //    stdout.flush().unwrap();
    //}
}
