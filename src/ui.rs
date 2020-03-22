
use std::io::{Write,Stdout};
use termion;
use termion::event::{Event, Key, MouseEvent};
use termion::input::{MouseTerminal, TermRead};
use termion::raw::IntoRawMode;
use termion::raw::RawTerminal;
use std::collections::HashMap;

use crate::logic::Level;

pub enum CursorPos{
    Pattern{i: isize, x: isize, y: isize},
    Replace{i: isize},
    Menu{last_i: isize},
}

impl CursorPos {
    pub fn up(&self, lvl: &Level) -> Self{
        use CursorPos::*;
        match *self {
            Pattern{i, x, y} => Pattern{i, x, y: (y as usize).saturating_sub(1) as isize},
            Replace{i} => Pattern{i: i, x:1, y:2},
            Menu{last_i} => Replace{i: last_i},
        }
    }
    pub fn down(&self, lvl: &Level) -> Self{
        use CursorPos::*;
        match *self {
            Pattern{i, x, y} if y < 2 => Pattern{i, x, y: y+1},
            Pattern{i, x, y}  => Replace{i},
            Replace{i} => Menu{last_i: i},
            Menu{last_i} => Menu{last_i},
        }
    }
    pub fn left(&self, lvl: &Level) -> Self{
        use CursorPos::*;
        let len = lvl.auto.rules.len() as isize;
        match *self {
            Pattern{i, x:0, y} => Pattern{i: (len+i-1)%len, x:2, y},
            Pattern{i, x, y} => Pattern{i, x:x-1, y},
            Replace{i} => Replace{i: (len+i-1)%len},
            Menu{last_i} => Menu{last_i},
        }
    }
    pub fn right(&self, lvl: &Level) -> Self{
        use CursorPos::*;
        let len = lvl.auto.rules.len() as isize;
        match *self {
            Pattern{i, x:2, y} => Pattern{i: (i+1)%len, x:0, y},
            Pattern{i, x, y} => Pattern{i, x:x+1, y},
            Replace{i} => Replace{i: (i+1)%len},
            Menu{last_i} => Menu{last_i},
        }
    }
}
const DOWN_RIGHT: &'static str= "╔";
const DOWN_LEFT: &'static str= "╗";
const UP_RIGHT: &'static str= "╚";
const UP_LEFT: &'static str= "╝";
const VERTICAL: &'static str= "║";
const VERTICAL_RIGHT: &'static str= "╠";
const VERTICAL_LEFT: &'static str= "╣";
const HORIZONTAL: &'static str= "═";
const UP_HORIZONTAL: &'static str= "╩";
const DOWN_HORIZONTAL: &'static str= "╦";
const VERTICAL_HORIZONTAL: &'static str= "╬";

pub type Out = MouseTerminal<RawTerminal<Stdout>>;

pub struct UIState{
    pub cursor: CursorPos,
}

impl UIState{
    pub fn draw_state_top(out: &mut Out, w: usize){
        for _ in 0..3 {
            write!(out, "{}", DOWN_RIGHT).unwrap();
            for i in 0..w {
                write!(out, "{}", HORIZONTAL).unwrap();
            }
            write!(out, "{} ", DOWN_LEFT).unwrap();
        }
    }

    pub fn draw_state_box_center(out: &mut Out, w: usize, h: usize){
        for _ in 0..3 {
        write!(out, "{}", VERTICAL).unwrap();
            for i in 0..w {
                write!(out, " ").unwrap();
            }
            write!(out, "{} ", VERTICAL).unwrap();
        }
    }
    
    pub fn draw_state_bot(out: &mut Out, w: usize){
        for _ in 0..3 {
            write!(out, "{}", UP_RIGHT).unwrap();
            for i in 0..w {
                write!(out, "{}", HORIZONTAL).unwrap();
            }
            write!(out, "{} ", UP_LEFT).unwrap();
        }
    }
    
    pub fn draw_rule_box_top(out: &mut Out, n: usize){
        write!(out, "{}", DOWN_RIGHT).unwrap();
        for i in 0..n {
            write!(out, "{}{}{}", HORIZONTAL,HORIZONTAL,HORIZONTAL).unwrap();
            if i != n-1 { write!(out,"{}",DOWN_HORIZONTAL).unwrap();}
        }
        write!(out, "{}", DOWN_LEFT).unwrap();
    }
    
    pub fn draw_rule_box_center(out: &mut Out, n: usize){
        write!(out, "{}", VERTICAL).unwrap();
        for i in 0..n {
            write!(out, "   {}",VERTICAL).unwrap();
        }
    }

    pub fn draw_rule_box_bot(out: &mut Out, n: usize){
        write!(out, "{}", UP_RIGHT).unwrap();
        for i in 0..n {
            write!(out, "{}▼{}", DOWN_HORIZONTAL,DOWN_HORIZONTAL).unwrap();
            if i != n-1 { write!(out,"{}",UP_HORIZONTAL).unwrap();}
        }
        write!(out, "{}", UP_LEFT).unwrap();
    }
    
    pub fn draw_replacement_box_center(out: &mut Out, n: usize){
        for i in 0..n {
            write!(out, " {} {}",VERTICAL,VERTICAL).unwrap();
        }
    }
    
    pub fn draw_replacement_box_bot(out: &mut Out, n: usize){
        for i in 0..n {
            write!(out, " {}{}{}", UP_RIGHT,HORIZONTAL,UP_LEFT).unwrap();
        }
    }    

    pub fn draw_rule_box(out: &mut Out, x: u16, y:u16, n: usize){
        write!(out, "{}", termion::cursor::Goto(x, y)).unwrap();
        Self::draw_rule_box_top(out, n);
        for i in 1..=3 {
            write!(out, "{}", termion::cursor::Goto(x, y+i)).unwrap();
            Self::draw_rule_box_center(out, n);
        }
        write!(out, "{}", termion::cursor::Goto(x, y+4)).unwrap();
        Self::draw_rule_box_bot(out, n);
        write!(out, "{}", termion::cursor::Goto(x, y+5)).unwrap();
        Self::draw_replacement_box_center(out,n);
        write!(out, "{}", termion::cursor::Goto(x, y+6)).unwrap();
        Self::draw_replacement_box_bot(out,n);
    }  

    fn draw_state_box(out: &mut Out, x: u16, y:u16, w: usize, h: usize){
        write!(out, "{}", termion::cursor::Goto(x, y)).unwrap();
        Self::draw_state_top(out, w);
        for i in 0_u16..=(h as u16) {
            write!(out, "{}", termion::cursor::Goto(x, y+i+1)).unwrap();
            Self::draw_state_box_center(out, w, h);
        } 
        write!(out, "{}", termion::cursor::Goto(x, y+2+(h as u16))).unwrap();
        Self::draw_state_bot(out, w);
    }

    pub fn set_cursor(&self, out: &mut Out, lvl: &Level){
        let (x,y) = match self.cursor {
            CursorPos::Pattern{i, x, y} => (self.rules_start(lvl).0 as isize + i*4 + x +1, self.rules_start(lvl).1 as isize + 1+y),
            CursorPos::Replace{i} => (self.rules_start(lvl).0 as isize + i*4+2,7),
            CursorPos::Menu{last_i} => (1,1),
            };
        write!(out, "{}", termion::cursor::Goto(x as u16, y as u16)).unwrap();
    }
    pub fn rules_start(&self, lvl: &Level) -> (u16, u16){
        let (w,h) = termion::terminal_size().unwrap();
        let rule_boxes = lvl.auto.rules.len();
        let rules_len :usize = rule_boxes*4 + 1;
        return (((w-rules_len as u16)/2) as u16, 2)
    }

    pub fn draw(&self, out: &mut Out, lvl: &Level){
        write!(out, "{}",termion::clear::All).unwrap();
        let (w,h) = termion::terminal_size().unwrap();
        let rule_boxes = lvl.auto.rules.len();
        let state_len = lvl.start.w*3 + 8;
        let rules_len :usize = rule_boxes*4 + 1;
        if w < rules_len as u16{ return Self::draw_terminal_to_small(out); }
        if w < state_len as u16{ return Self::draw_terminal_to_small(out); }
        let (rule_x,rule_y) = self.rules_start(lvl);
        Self::draw_rule_box(out, rule_x, rule_y, rule_boxes);
        Self::draw_state_box(out, ((w-state_len as u16)/2) as u16, 9, lvl.start.w, lvl.start.h);
        self.set_cursor(out, lvl);
        out.flush().unwrap();
    }
    
    pub fn draw_terminal_to_small(out: &mut Out){
        write!(out, "{}",termion::clear::All).unwrap();
        let (w,h) = termion::terminal_size().unwrap();
        let warn = "resize term";
        write!(out, "{}{}", termion::cursor::Goto((w-(warn.len()) as u16)/2, h/2), warn).unwrap();
        out.flush().unwrap();
    }
}