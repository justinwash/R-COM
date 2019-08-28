extern crate crossterm;
extern crate rand;

use self::crossterm::{ execute, input, style, AsyncReader, 
    Clear, ClearType, Color, Colorize, Command, Crossterm, Goto, 
    InputEvent, KeyEvent, Output, PrintStyledFont, RawScreen, Show, TerminalCursor
};

use std::io::{stdout, Write};

fn main() {
    let _raw = RawScreen::into_raw_mode();
    let crossterm = Crossterm::new();

    init_ui();
    init_map();

    let origin = get_cursor_origin(crossterm.cursor());
    let game_state = Mode::Select(Select::None);

    let mut stdin = crossterm.input().read_async();

    let squad = vec![
        Soldier::new(String::from("Jinpachi"), Position { x: 2, y: 2 }),
        Soldier::new(String::from("Heihachi"), Position { x: 2, y: 3 }),
        Soldier::new(String::from("Kazumi"), Position { x: 2, y: 4 }),
        Soldier::new(String::from("Kazuya"), Position { x: 3, y: 2 }),
        Soldier::new(String::from("Jun"), Position { x: 3, y: 3 }),
        Soldier::new(String::from("Jin"), Position { x: 3, y: 4 }),
    ];

    loop {
        match game_state {
            Mode::Deploy => {
                update_cursor(&mut stdin, crossterm.cursor(), origin);

            }
            Mode::Select(Select::None) => {
                update_cursor(&mut stdin, crossterm.cursor(), origin);
            }
            _ => { }
        }
    }
}

fn init_ui() {
    execute!(
        stdout(),
        Clear(ClearType::All),
        Goto(0, 0),
        PrintStyledFont(style(format!("{}", BORDERS.join("\n\r"))).with(Color::White)),
        Show
    ).unwrap();
}

fn init_map() {
    // max map dimensions: 12x48
    // execute!(stdout(), Goto(3, 3), PrintStyledFont(style(format!("{}", crossterm.cursor().pos()).with(Color::White)), Show).unwrap();
}

fn get_cursor_origin(cursor: TerminalCursor) -> (u16, u16) {
    execute!(stdout(), Goto(0, 0),Show).unwrap();
    let origin = cursor.pos();
    execute!(stdout(), Goto(2, 2),Show).unwrap();
    return origin;
}

fn update_cursor(input: &mut AsyncReader, mut cursor: TerminalCursor, origin: (u16, u16)) {
    let pressed_key = input.next();

    if let Some(InputEvent::Keyboard(key)) = pressed_key {
        let (cursor_x, cursor_y) = cursor.pos();
        match key {
            KeyEvent::Up => { if cursor_y > origin.1 + 2 { cursor.move_up(1); } }
            KeyEvent::Down => { if cursor_y < origin.1 + 14 { cursor.move_down(1); } }
            KeyEvent::Left => { if cursor_x > 2 { cursor.move_left(1); } }
            KeyEvent::Right => { if cursor_x < 50 { cursor.move_right(1); } }
            _ => { },
        };
    }
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

struct Loop { }
impl Loop {
    pub fn select(mut self) {
        loop {

        }
    }
}

enum Mode {
    Deploy,
    EnemyTurn,
    Select(Select),
    Action(Action),
}

enum Select {
    None,
    Terrain,
    Object,
    Enemy,
    Soldier
}

enum Action {
    Move,
    Fire,
    Sprint,
    UseItem,
    HunkerDown,
    Reload,
    ChangeWeapon
}

struct Position {
    x: u16,
    y: u16
}


struct Soldier {
    name: String,
    hp: u16,
    pos: Position
}

impl Soldier {
    fn new(name: String, pos: Position) -> Soldier {
         return Soldier {
             name: name,
             hp: 10,
             pos: pos
         }
    }
}

