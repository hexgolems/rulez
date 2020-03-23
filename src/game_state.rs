use crate::playing_ui::Out;
use crate::playing_ui::PlayingUI;
use termion::event::Event;

pub enum GameState {
    Quit,
    Menu,
    Play(PlayingUI),
    Success,
}

impl GameState {
    pub fn tick(&mut self) -> Option<Self> {
        match self {
            Self::Quit => return None,
            Self::Menu => return None,
            Self::Play(ui) => {
                return if ui.update() {
                    Some(Self::Success)
                } else {
                    None
                }
            }
            Self::Success => return None,
        }
    }

    pub fn event(&mut self, event: Event) -> Option<Self> {
        match self {
            Self::Quit => return None,
            Self::Menu => return None,
            Self::Play(ui) => return ui.event(event),
            Self::Success => return None,
        }
    }

    pub fn draw(&mut self, out: &mut Out) {
        match self {
            Self::Quit => return,
            Self::Menu => return,
            Self::Play(ui) => ui.draw(out),
            Self::Success => return,
        }
    }
}
