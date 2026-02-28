use super::GameError;
use super::GameState;
use super::Scenario;

pub enum CommandOutcome {
    Continue,
    Quit,
    Victory,
    Defeat,
}

pub trait GameCommand {
    fn execute(
        &self,
        scenario: &Scenario,
        state: &mut GameState,
    ) -> Result<CommandOutcome, GameError>;
}

pub struct LookCommand {}

impl GameCommand for LookCommand {
    fn execute(
        &self,
        _scenario: &Scenario,
        state: &mut GameState,
    ) -> Result<CommandOutcome, GameError> {
        state.print_current_scene();
        Ok(CommandOutcome::Continue)
    }
}
pub struct ChooseCommand {
    pub choice_id: usize,
}

impl GameCommand for ChooseCommand {
    fn execute(
        &self,
        scenario: &Scenario,
        state: &mut GameState,
    ) -> Result<CommandOutcome, GameError> {
        if !state.is_choice_ok(self.choice_id) {
            return Err(GameError::InvalidChoice);
        }

        if !state.has_necessary_item(self.choice_id) {
            return Err(GameError::MissingItem(String::new()));
        }

        state.change_scene(scenario, self.choice_id);
        state.update();

        if state.is_ending() {
            if state.current_hp <= 0 {
                return Ok(CommandOutcome::Defeat);
            }
            return Ok(CommandOutcome::Victory);
        }

        Ok(CommandOutcome::Continue)
    }
}

pub struct InventoryCommand {}

impl GameCommand for InventoryCommand {
    fn execute(
        &self,
        _scenario: &Scenario,
        state: &mut GameState,
    ) -> Result<CommandOutcome, GameError> {
        state.print_inventory();
        Ok(CommandOutcome::Continue)
    }
}

pub struct StatusCommand {}

impl GameCommand for StatusCommand {
    fn execute(
        &self,
        _scenario: &Scenario,
        state: &mut GameState,
    ) -> Result<CommandOutcome, GameError> {
        state.print_status();
        Ok(CommandOutcome::Continue)
    }
}

pub struct QuitCommand {}

impl GameCommand for QuitCommand {
    fn execute(
        &self,
        _scenario: &Scenario,
        _state: &mut GameState,
    ) -> Result<CommandOutcome, GameError> {
        Ok(CommandOutcome::Quit)
    }
}
