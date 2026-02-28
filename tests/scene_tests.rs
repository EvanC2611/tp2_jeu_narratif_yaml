use tp2_jeu_narratif_yaml::{
    GameError, GameState, Scenario,
    commands::{ChooseCommand, CommandOutcome, GameCommand},
};

fn setup_scenario() -> Scenario {
    let yaml = r#"
start_scene: entrance
initial_hp: 10
scenes:
  - id: entrance
    title: Porte Principale
    text: La pluie frappe les vitres. Devant vous, le complexe est ouvert.
    choices:
      - label: Entrer dans le hall
        next: hall
      - label: Renoncer et partir dans la rue
        next: collapse
      - label: Forcer l'acces au toit
        next: roof
        required_item: badge
      - label: Entrer dans le hall
        next: roof

  - id: collapse
    title: Effondrement
    text: Une onde de choc traverse le batiment. Le plafond s'effondre.
    hp_delta: -12
    ending: defeat

  - id: roof
    title: Toit
    text: Vous activez la balise de secours. Un drone vous repere.
    ending: victory
"#;
    serde_yaml::from_str(yaml).unwrap()
}

#[test]
fn test_invalid_choice() {
    let scenario = setup_scenario();
    let mut state = GameState::new(&scenario);
    let cmd = ChooseCommand { choice_id: 99 };
    let res = cmd.execute(&scenario, &mut state);
    assert!(matches!(res, Err(GameError::InvalidChoice)));
}

#[test]
fn test_nominal_path_to_victory() {
    let scenario = setup_scenario();
    let mut state = GameState::new(&scenario);
    let cmd = ChooseCommand { choice_id: 3 };
    let res = cmd.execute(&scenario, &mut state).unwrap();
    assert!(matches!(res, CommandOutcome::Victory));
}

#[test]
fn test_conditional_choice_missing_item() {
    let scenario = setup_scenario();
    let mut state = GameState::new(&scenario);
    let cmd = ChooseCommand { choice_id: 2 };
    let res = cmd.execute(&scenario, &mut state);
    assert!(matches!(res, Err(GameError::MissingItem(_))));
}

#[test]
fn test_hp_loss_game_over() {
    let scenario = setup_scenario();
    let mut state = GameState::new(&scenario);
    let cmd = ChooseCommand { choice_id: 1 };
    let res = cmd.execute(&scenario, &mut state).unwrap();
    assert!(matches!(res, CommandOutcome::Defeat));
    assert!(state.current_hp <= 0);
}

#[test]
fn test_invalid_yaml_validation() {
    let yaml = "start_scene: unknown\ninitial_hp: 10\nscenes: []";
    let scenario: Scenario = serde_yaml::from_str(yaml).unwrap();
    assert!(matches!(
        scenario.validate_scenario(),
        Err(GameError::InvalidScene)
    ));
}
