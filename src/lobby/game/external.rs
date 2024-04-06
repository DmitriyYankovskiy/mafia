use super::{
    character::{self, CharacterInfo, Num},
    role::Role,
};


#[derive(Debug, serde::Serialize)]
pub struct StartInfo {
    pub num: Num,
    pub cnt_characters: usize,
    pub role: Role,
}