use super::{
    character::Num,
    role::Role,
};

pub mod incom {
    use super::Num;

    #[derive(Debug, serde::Deserialize, Clone)]
    pub enum M {
        Action {
            target: Num,
        }, 
        Accuse {
            target: Num,
        },
        Vote,
    }
}
pub mod outgo {
    use super::{Num, Role};

    #[derive(Debug, serde::Serialize, Clone)]
    pub enum M {
        Start {
            num: Num,
            cnt_characters: usize,
            role: Role,
        },
        Time(TimeInfo),
        Die {num: Num, time: u64},
        Vote {from: Num},
        Check {num: Num, res: bool},
        Accuse {num: Num},
        Next {num: Num},
    }

    #[derive(Debug, serde::Serialize, Clone)]
    #[serde(tag = "phase")]
    pub enum TimeInfo {
        Night{time: u64},
        Sunrise, 
        Discussion,
        Voting{candidates: Vec<Num>},
        Sunset,
    }
}