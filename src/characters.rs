pub struct Character {
    player: Player,
    role: Role,
    num: usize,
}


#[derive(Copy, Clone, Debug, Hash, PartialEq, Eq)]
pub enum Role {
    Civilian,
    Mafia,
    Sheriff,
    Maniac,
}