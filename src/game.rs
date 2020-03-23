use crate::game_state::GameState;
use crate::level::Level;
use crate::playing_ui::{Out, PlayingUI};
use termion::event::Event;

pub struct Game {
    pub levels: Vec<Level>,
    pub out: Out,
    pub state: GameState,
    pub level: usize,
}

impl Game {
    pub fn tick(&mut self) {
        if let GameState::Success = self.state {
            let lvl = self.levels[self.level].clone();
            self.level += 1;
            let ui = PlayingUI::new(lvl);
            self.state = GameState::Play(ui);
        }
        if let Some(newstate) = self.state.tick() {
            self.state = newstate;
        }
        self.state.draw(&mut self.out);
    }

    pub fn event(&mut self, event: Event) {
        if let Some(state) = self.state.event(event) {
            self.state = state;
        }
        self.state.draw(&mut self.out);
    }
}
