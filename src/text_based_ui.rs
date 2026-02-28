use super::commands::CommandOutcome;
use super::{GameState, Scenario};

use super::ui::UI;

use super::commands::{
    ChooseCommand, GameCommand, InventoryCommand, LookCommand, QuitCommand, StatusCommand,
};
use super::errors::ParseError;

pub struct TBUI {}

impl UI for TBUI {
    fn parse_command(line: &str) -> Result<Box<dyn GameCommand>, ParseError> {
        let parts: Vec<&str> = line.trim().split_whitespace().collect();

        if parts.is_empty() {
            return Err(ParseError::InvalidCommand);
        }

        match parts[0] {
            "look" => Ok(Box::new(LookCommand {})),
            "inventory" => Ok(Box::new(InventoryCommand {})),
            "status" => Ok(Box::new(StatusCommand {})),
            "quit" => Ok(Box::new(QuitCommand {})),
            "choose" => {
                if parts.len() < 2 {
                    return Err(ParseError::InvalidCommand);
                }

                Ok(Box::new(ChooseCommand {
                    choice_id: parts[1].parse::<usize>().unwrap(),
                }))
            }
            _ => return Err(ParseError::InvalidCommand),
        }
    }
}

impl TBUI {
    pub fn run(scenario: Scenario) {
        let mut state = GameState::new(&scenario);
        state.print_current_scene();

        loop {
            let mut input = String::new();
            std::io::stdin().read_line(&mut input).unwrap();

            match TBUI::parse_command(&input) {
                Ok(cmd) => match cmd.execute(&scenario, &mut state) {
                    Ok(CommandOutcome::Continue) => {}
                    Ok(CommandOutcome::Victory) => {
                        println!("Victory");
                        break;
                    }
                    Ok(CommandOutcome::Defeat) => {
                        println!("Defeat");
                        break;
                    }
                    Ok(CommandOutcome::Quit) => {
                        break;
                    }
                    Err(e) => println!("Erreur: {:?}", e),
                },
                Err(e) => println!("Bad command: {:?}", e),
            }
        }
    }
}
