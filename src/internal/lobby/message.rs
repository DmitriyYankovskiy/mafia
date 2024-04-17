use super::game::message;

pub mod incom {
    use super::message;
    #[derive(Debug, serde::Deserialize, Clone)]
    pub enum M {
        Game(message::incom::M),
    }
}
pub mod outgo {
    use super::message;

    #[derive(Debug, serde::Serialize, Clone)]
    pub enum M {
        Game(message::outgo::M),
    }
}