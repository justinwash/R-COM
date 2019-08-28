extern crate crossterm;
use std::io::{stdout, Write};

use self::crossterm::{ 
  execute, 
  AsyncReader, 
  Goto, 
  InputEvent, 
  KeyEvent, 
  Show, 
  TerminalCursor
};

pub fn get_cursor_origin(cursor: TerminalCursor) -> (u16, u16) {
    execute!(stdout(), Goto(0, 0),Show).unwrap();
    let origin = cursor.pos();
    execute!(stdout(), Goto(2, 2),Show).unwrap();
    return origin;
}

pub fn update_cursor(input: &mut AsyncReader, mut cursor: TerminalCursor, origin: (u16, u16)) {
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