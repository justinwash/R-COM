extern crate crossterm;
extern crate rand;

use self::crossterm::{ execute, input, style, AsyncReader, 
    Clear, ClearType, Color, Colorize, Command, Crossterm, Goto, 
    InputEvent, KeyEvent, Output, PrintStyledFont, RawScreen, Show,
};

use std::io::{stdout, Write};


fn main() {
    init();

    // screen has to be in raw mode in order for the key presses not to be printed to the screen.
    let _raw = RawScreen::into_raw_mode();
    let crossterm = Crossterm::new();

    let mut stdin = crossterm.input().read_async();

    // start the game loop;
    loop {
        let pressed_key = stdin.next();

        if let Some(InputEvent::Keyboard(key)) = pressed_key {
            match key {
                KeyEvent::Up => { crossterm.cursor().move_up(1); }
                KeyEvent::Down => { crossterm.cursor().move_down(1); }
                KeyEvent::Left => { crossterm.cursor().move_left(1); }
                KeyEvent::Right => { crossterm.cursor().move_right(1); }
                _ => { },
            };
        }
    }
}

fn init() {
    execute!(
        stdout(),
        Clear(ClearType::All),
        Goto(0, 0),
        PrintStyledFont(style(format!("{}", BORDERS.join("\n\r"))).with(Color::Red)),
        Goto(1, 14),
        PrintStyledFont(style(format!("{}", COMMANDS.join("\n\r"))).with(Color::White)),
        Show
    );
}



const BORDERS: [&str; 16] = [
    "╔═╡ R-COM ╞════════════════════════════════════════════════════════════╗",
    "║                                                                      ║",
    "║                                                                      ║",
    "║                                                                      ║",
    "║                                                                      ║",
    "║                                                                      ║",
    "║                                                                      ║",
    "║                                                                      ║",
    "║                                                                      ║",
    "║                                                                      ║",
    "║                                                                      ║",
    "║                                                                      ║",
    "║                                                                      ║",
    "╠═══╡ Commands ╞═══════════════════════════════════════════════════════╣",
    "║                                                                      ║",
    "╚══════════════════════════════════════════════════════════════════════╝",
];

const COMMANDS: [&str; 1] = [
        " Move    Aim     Use Item     Hunker Down     Reload    Switch Weapon",
];
