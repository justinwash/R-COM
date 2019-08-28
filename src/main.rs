extern crate crossterm;
extern crate rand;

use self::crossterm::{ Crossterm, RawScreen };

mod soldier;
mod utilities;
mod state;
mod ui;
mod cursor;
mod map;

use soldier::{ Soldier };
use utilities::{ Position };
use state::{ Mode, Select };
use ui::{ init_ui };
use cursor::{ get_cursor_origin, update_cursor };
use map::{ init_map };

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
