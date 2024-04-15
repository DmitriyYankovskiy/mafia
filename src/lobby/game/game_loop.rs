use super::character::Num;

#[derive(Clone, Copy)]
pub struct Candidate {
    pub num: Num,
    pub cnt_votes: usize,
}

impl Candidate {
    pub fn new(num: Num) -> Self {
        Self {
            num,
            cnt_votes: 0,
        }
    }
}