extern crate crossterm;
use std::io::{stdout, Write};

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

const BORDERS: [&str; 19] = [
    "╔═╡ R-COM ╞═════════════════════════════════════════╦═╡ Status ╞══════╗",
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
    "║                                                   ║                 ║",
    "╠═══╡ Commands ╞════════════════════════════════════╩═════════════════╣",
    "║                                                                     ║",
    "╚═════════════════════════════════════════════════════════════════════╝",
];