use crate::level::Level;
use termion::event::Event;
use crate::game_state::{GameState};
use crate::playing_ui::Out;

pub struct Game {
    pub levels: Vec<Level>,
    pub out: Out,
    pub state: GameState,
}

impl Game{
    pub fn tick(&mut self){
        if let Some(newstate) = self.state.tick(){
            self.state = newstate;
        }
        self.state.draw(&mut self.out);
    }
    
    pub fn event(&mut self, event: Event) {
        self.state.event(event);
        self.state.draw(&mut self.out);
    }
}