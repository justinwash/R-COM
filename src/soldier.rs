use utilities::{ Position };

pub struct Soldier {
    pub name: String,
    pub hp: u16,
    pub pos: Position
}

impl Soldier {
    pub fn new(name: String, pos: Position) -> Soldier {
         return Soldier {
             name: name,
             hp: 10,
             pos: pos
         }
    }
}