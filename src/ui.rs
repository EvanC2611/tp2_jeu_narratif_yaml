use super::commands::GameCommand;
use super::errors::ParseError;
pub trait UI {
    fn parse_command(line: &str) -> Result<Box<dyn GameCommand>, ParseError>;
}
