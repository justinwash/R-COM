extern crate crossterm;
extern crate rand;

use self::crossterm::{ Crossterm, RawScreen };

mod soldier;
mod squad;
mod utilities;
mod state;
mod ui;
mod cursor;
mod map;

use soldier::{ Soldier };
use squad::{ Squad };
use utilities::{ Position };
use state::{ Mode, Select };
use ui::{ init_ui, draw_details, clear_details };
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

    let squad = Squad::new();
    squad.draw();

    let mut highlighted_obj_name = "";

    loop {
        let cursor_pos = update_cursor(&mut stdin, crossterm.cursor(), &origin);

        match game_state {
            Mode::Select(Select::None) => {
                for soldier in &squad.members {
                    if cursor_pos == soldier.pos && soldier.name != highlighted_obj_name {
                        highlighted_obj_name = &soldier.name;
                        draw_details(soldier);
                    }
                }
            }
            _ => { }
        }
    }
}
