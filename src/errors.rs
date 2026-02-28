#[derive(Debug, PartialEq, Eq, Clone)]
pub enum GameError {
    InvalidChoice,
    MissingItem(String),
    InvalidScene,
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum ParseError {
    InvalidCommand,
}
