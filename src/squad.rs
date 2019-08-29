use soldier::Soldier;
use utilities::Position;

extern crate crossterm;
use std::io::{stdout, Write};

use self::crossterm::{ 
  execute, 
  style,
  Color, 
  Goto, 
  PrintStyledFont,
  Show
};

pub struct Squad {
  pub members: Vec<Soldier>
}

impl Squad {
  pub fn new() -> Squad {
    return Squad {
      members: vec![
        Soldier::new(String::from("Jinpachi"), Position { x: 2, y: 2 }),
        Soldier::new(String::from("Heihachi"), Position { x: 2, y: 3 }),
        Soldier::new(String::from("Kazumi"), Position { x: 2, y: 4 }),
        Soldier::new(String::from("Kazuya"), Position { x: 3, y: 2 }),
        Soldier::new(String::from("Jun"), Position { x: 3, y: 3 }),
        Soldier::new(String::from("Jin"), Position { x: 3, y: 4 }),
      ]
    }
  }

  pub fn draw(&self) {
    for soldier in &self.members {
      execute!(stdout(), 
        Goto(soldier.pos.x, soldier.pos.y), 
        PrintStyledFont(style("@").with(Color::White)), 
        Show
        ).unwrap();
    }
  }
}