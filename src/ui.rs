extern crate crossterm;
use std::io::{stdout, Write};

use soldier::Soldier;
use utilities::Position;

use self::crossterm::{ 
  execute, 
  style, 
  Clear, 
  ClearType, 
  Color, 
  Goto, 
  PrintStyledFont,
  Show
};

pub fn init_ui() {
    execute!(
        stdout(),
        Clear(ClearType::All),
        Goto(0, 0),
        PrintStyledFont(style(format!("{}", BORDERS.join("\n\r"))).with(Color::White)),
        Show
    ).unwrap();
}

pub fn draw_details(soldier: &Soldier) {
  execute!(
        stdout(),
        Goto(54,1),
        PrintStyledFont(style(format!("{}", "Name:")).with(Color::White)),
        Goto(55,2),
        PrintStyledFont(style(format!("{}", "          ")).with(Color::White)),
        Goto(55,2),
        PrintStyledFont(style(format!("{}", soldier.name)).with(Color::White)),
        Goto(soldier.pos.x, soldier.pos.y),
        Show
    ).unwrap();
}

pub fn clear_details(initial_position: &Position) {
  execute!(
        stdout(),
        Goto(54,1),
        PrintStyledFont(style(format!("{}", "          ")).with(Color::White)),
        Goto(55,2),
        PrintStyledFont(style(format!("{}", "          ")).with(Color::White)),
        Goto(initial_position.x,initial_position.y),
        Show
    ).unwrap();
}













const BORDERS: [&str; 19] = [
    "╔═╡ R-COM ╞═════════════════════════════════════════╦═╡ Status ╞══════╗",
    "║                                                   ║ Name:           ║",
    "║                                                   ║                 ║",
    "║                                                   ║                 ║",
    "║                                                   ║                 ║",
    "║                                                   ║                 ║",
    "║                                                   ║                 ║",
    "║                                                   ║                 ║",
    "║                                                   ║                 ║",
    "║                                                   ║                 ║",
    "║                                                   ║                 ║",
    "║                                                   ║                 ║",
    "║                                                   ║                 ║",
    "║                                                   ║                 ║",
    "║                                                   ║                 ║",
    "║                                                   ║                 ║",
    "╠═══╡ Commands ╞════════════════════════════════════╩═════════════════╣",
    "║                                                                     ║",
    "╚═════════════════════════════════════════════════════════════════════╝",
];