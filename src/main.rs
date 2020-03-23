extern crate itertools;
extern crate termion;
#[macro_use]
extern crate serde_derive;
extern crate glob;
extern crate ron;

mod automaton;
mod cursor_pos;
mod field;
mod game;
mod game_state;
mod level;
mod rule;
mod playing_ui;

use game::{Game};
use game_state::{GameState};
use glob::glob;
use level::Level;
use std::fs::File;
use std::io::prelude::*;
use std::io::{stdin, stdout};
use std::sync::{Arc, RwLock};
use std::thread;
use std::time;
use termion::input::{MouseTerminal, TermRead};
use termion::raw::IntoRawMode;
use termion::screen::AlternateScreen;
use playing_ui::{PlayingUI};

fn animation(game: Arc<RwLock<Game>>) {
    loop {
        let ten_millis = time::Duration::from_millis(500);
        thread::sleep(ten_millis);
        game.write().unwrap().tick();
        if let GameState::Quit = game.read().unwrap().state {
            break;
        }
    }
}

fn load_level(path: &str) -> Level {
    let mut file = File::open(path).unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();
    return ron::de::from_str(&contents).unwrap();
}

fn main() {
    let levels = glob("level/level*")
        .unwrap()
        .map(|entry| load_level(&entry.unwrap().into_os_string().into_string().unwrap()))
        .collect::<Vec<_>>();
    let lvl = levels[0].clone();
    let ui = PlayingUI::new(lvl);
    let screen = AlternateScreen::from(stdout().into_raw_mode().unwrap());
    let out = MouseTerminal::from(screen);
    let game = Game {
        levels,
        out,
        state: GameState::Play(ui),
    };

    let game = Arc::new(RwLock::new(game));
    let game2 = game.clone();
    thread::spawn(|| animation(game2));

    for c in stdin().events() {
        let evt = c.unwrap();
        game.write().unwrap().event(evt);
        if let GameState::Quit = game.read().unwrap().state {
            break;
        }
    }
}
