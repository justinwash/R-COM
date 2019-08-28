pub enum Mode {
    Deploy,
    EnemyTurn,
    Select(Select),
    Action(Action),
}

pub enum Select {
    None,
    Terrain,
    Object,
    Enemy,
    Soldier
}

pub enum Action {
    Move,
    Fire,
    Sprint,
    UseItem,
    HunkerDown,
    Reload,
    ChangeWeapon
}