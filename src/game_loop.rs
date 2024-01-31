struct GameLoop {
    stage: 
}

pub enum Stage {
    Night,
    Sunrise,
    Discussion(),
    Voting(),
    Sunset,
}

struct Candidate {
    num: usize,
    cnt_vote: usize,
}

struct Discussion {
    num: usize,
    candidates: Vec<Candidate>,
}