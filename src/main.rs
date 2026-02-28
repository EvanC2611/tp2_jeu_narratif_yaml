use tp2_jeu_narratif_yaml::{Scenario, text_based_ui::TBUI};

fn main() {
    let scenario = match Scenario::parse_scene("./story.yaml") {
        Ok(s) => s,
        Err(_) => {
            panic!("No file found")
        }
    };

    if scenario.validate_scenario().is_err() {
        panic!("Invalid scenario")
    }

    TBUI::run(scenario);
}
