
use std::io::{Write,Stdout};
use termion;
use termion::input::{MouseTerminal};
use termion::raw::RawTerminal;
use termion::screen::AlternateScreen;

use crate::logic::{Level,Field, Rule};
use std::process::exit;

const DOWN_RIGHT: &'static str= "╔";
const DOWN_LEFT: &'static str= "╗";
const UP_RIGHT: &'static str= "╚";
const UP_LEFT: &'static str= "╝";
const VERTICAL: &'static str= "║";
//const VERTICAL_RIGHT: &'static str= "╠";
//const VERTICAL_LEFT: &'static str= "╣";
const HORIZONTAL: &'static str= "═";
const UP_HORIZONTAL: &'static str= "╩";
const DOWN_HORIZONTAL: &'static str= "╦";
//const VERTICAL_HORIZONTAL: &'static str= "╬";

pub enum CursorPos{
    Pattern{i: isize, x: isize, y: isize},
    Replace{i: isize},
    State{last_i: isize},
}

impl CursorPos {
    pub fn in_state(&self) -> bool{
        match self {
            CursorPos::State{..} => true,
            _ => false,
        }
    }

    pub fn up(&self, _lvl: &Level) -> Self{
        use CursorPos::*;
        match *self {
            Pattern{i, x, y} => Pattern{i, x, y: (y as usize).saturating_sub(1) as isize},
            Replace{i} => Pattern{i: i, x:1, y:2},
            State{last_i} => Replace{i: last_i},
        }
    }
    pub fn down(&self, _lvl: &Level) -> Self{
        use CursorPos::*;
        match *self {
            Pattern{i, x, y} if y < 2 => Pattern{i, x, y: y+1},
            Pattern{i, ..}  => Replace{i},
            Replace{i} => State{last_i: i},
            State{last_i} => State{last_i},
        }
    }
    pub fn left(&self, lvl: &Level) -> Self{
        use CursorPos::*;
        let len = lvl.auto.rules.len() as isize;
        match *self {
            Pattern{i, x:0, y} => Pattern{i: (len+i-1)%len, x:2, y},
            Pattern{i, x, y} => Pattern{i, x:x-1, y},
            Replace{i} => Replace{i: (len+i-1)%len},
            State{last_i} => State{last_i},
        }
    }
    pub fn right(&self, lvl: &Level) -> Self{
        use CursorPos::*;
        let len = lvl.auto.rules.len() as isize;
        match *self {
            Pattern{i, x:2, y} => Pattern{i: (i+1)%len, x:0, y},
            Pattern{i, x,y } => Pattern{i, x:x+1, y},
            Replace{i} => Replace{i: (i+1)%len},
            State{last_i} => State{last_i},
        }
    }
}


pub type Out = MouseTerminal<AlternateScreen<RawTerminal<Stdout>>>;

pub struct UIState{
    pub cursor: CursorPos,
    pub animate: bool,
    pub cur_step: usize,
    pub field: Field,
}

impl UIState{
    pub fn new(lvl: &Level) -> Self{
        return Self{cursor: CursorPos::Pattern{i:0, x:0, y:0}, animate: false, cur_step:0, field: lvl.start.clone() }
    }
    
    pub fn toggle(&mut self, lvl: &mut Level){
        if self.cursor.in_state(){
            self.animate = !self.animate;
        } 
        else {
            self.set_char(&' ', lvl);
        }
    }

    pub fn set_char(&mut self, chr: &char, lvl: &mut Level){
        let mut buf = [0; 4];
        let chr = chr.encode_utf8(&mut buf).as_bytes()[0];
        match self.cursor{
            CursorPos::Pattern{i,x,y} => lvl.auto.rules[i as usize].pattern[(x+y*3) as usize] = chr,
            CursorPos::Replace{i} => lvl.auto.rules[i as usize].replace = chr,
            _ => {},
        }
        self.reset_field(lvl);
    }
    
    pub fn reset_field(&mut self, lvl: &Level){
        self.field = lvl.start.clone();
        self.cur_step = 0;
    }

    pub fn update(&mut self, lvl: &Level){
        if self.field != lvl.goal { 
            self.field = lvl.auto.step(&self.field);
            self.cur_step+=1;
        } else {
            exit(0x0);
        }
    }

    pub fn draw_rules(out: &mut Out, rules: &[Rule], x: usize, y: usize){
        write!(out, "{}", termion::cursor::Goto((x+1) as u16, y as u16)).unwrap();
        for (j, rule) in rules.iter().enumerate() {
            for i in 0..3 {
                write!(out, "{}", termion::cursor::Goto((x+1 + j*4) as u16, (y+i+1) as u16)).unwrap();
                out.write(&rule.pattern[i*3..i*3+3]).unwrap();
            }          
        }
    }

    pub fn draw_replacements(out: &mut Out, rules: &[Rule], x: usize, y: usize){
        write!(out, "{}", termion::cursor::Goto((x+1) as u16, y as u16)).unwrap();
        for (j, rule) in rules.iter().enumerate() {
            write!(out, "{}", termion::cursor::Goto((x+2 + j*4) as u16, (y+5) as u16)).unwrap();
            out.write(&[rule.replace]).unwrap();
        }
    }

    pub fn draw_field(out: &mut Out, field: &Field, x: usize, y: usize){
        write!(out, "{}", termion::cursor::Goto((x+1) as u16, y as u16)).unwrap();
        for i in 0..field.h{
            let line = &field.data[i];
            write!(out, "{}", termion::cursor::Goto((x+1) as u16, (y+i+1) as u16)).unwrap();
            out.write(line).unwrap();
        }
    }

    pub fn draw_state_top(out: &mut Out, w: usize){   
        let headlines = ["Start", "Step", "Goal"];
        for headline in headlines.iter() {
            write!(out, "{}", DOWN_RIGHT).unwrap();
            write!(out, "{}", headline).unwrap();
            for _i in 0..w-headline.len() {
                write!(out, "{}", HORIZONTAL).unwrap();
            }
            write!(out, "{} ", DOWN_LEFT).unwrap();
        }
    }

    pub fn draw_state_box_center(out: &mut Out, w: usize, _h: usize){
        for _ in 0..3 {
        write!(out, "{}", VERTICAL).unwrap();
            for _i in 0..w {
                write!(out, " ").unwrap();
            }
            write!(out, "{} ", VERTICAL).unwrap();
        }
    }
    
    pub fn draw_state_bot(out: &mut Out, w: usize){
        for _ in 0..3 {
            write!(out, "{}", UP_RIGHT).unwrap();
            for _i in 0..w {
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
        for _i in 0..n {
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
        for _i in 0..n {
            write!(out, " {} {}",VERTICAL,VERTICAL).unwrap();
        }
    }
    
    pub fn draw_replacement_box_bot(out: &mut Out, n: usize){
        for _i in 0..n {
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
        for i in 0_u16..(h as u16) {
            write!(out, "{}", termion::cursor::Goto(x, y+i+1)).unwrap();
            Self::draw_state_box_center(out, w, h);
        } 
        write!(out, "{}", termion::cursor::Goto(x, y+1+(h as u16))).unwrap();
        Self::draw_state_bot(out, w);
    }

    pub fn set_cursor(&self, out: &mut Out, lvl: &Level){
        let (x,y) = match self.cursor {
            CursorPos::Pattern{i, x, y} => (self.rules_start(lvl).0 as isize + i*4 + x +1, self.rules_start(lvl).1 as isize + 1+y),
            CursorPos::Replace{i} => (self.rules_start(lvl).0 as isize + i*4+2,7),
            CursorPos::State{..} => (1,1),
            };
        write!(out, "{}", termion::cursor::Goto(x as u16, y as u16)).unwrap();
    }
    pub fn rules_start(&self, lvl: &Level) -> (u16, u16){
        let (w,_h) = termion::terminal_size().unwrap();
        let rule_boxes = lvl.auto.rules.len();
        let rules_len :usize = rule_boxes*4 + 1;
        return (((w-rules_len as u16)/2) as u16, 2)
    }

    pub fn draw(&self, out: &mut Out, lvl: &Level){
        write!(out, "{}",termion::clear::All).unwrap();
        let (w,_h) = termion::terminal_size().unwrap();
        let rule_boxes = lvl.auto.rules.len();
        let state_len = lvl.start.w*3 + 8;
        let rules_len :usize = rule_boxes*4 + 1;
        if w < rules_len as u16{ return Self::draw_terminal_to_small(out); }
        if w < state_len as u16{ return Self::draw_terminal_to_small(out); }
        let (rule_x,rule_y) = self.rules_start(lvl);
        Self::draw_rule_box(out, rule_x, rule_y, rule_boxes);
        Self::draw_state_box(out, ((w-state_len as u16)/2) as u16, 9, lvl.start.w, lvl.start.h);
        Self::draw_rules(out, &lvl.auto.rules, rule_x as usize, rule_y as usize);
        Self::draw_replacements(out, &lvl.auto.rules, rule_x as usize, rule_y as usize);
        Self::draw_field(out, &lvl.start, ((w-state_len as u16)/2) as usize, 9);
        Self::draw_field(out, &self.field, (((w-state_len as u16)/2) as usize) + &lvl.start.w +3, 9);
        Self::draw_field(out, &lvl.goal, (((w-state_len as u16)/2) as usize) + 2*&lvl.start.w + 6, 9);
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