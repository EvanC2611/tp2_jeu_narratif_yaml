use crate::{Scenario, Scene};

pub struct GameState {
    current_scene: Scene,
    pub current_hp: i32,
    inventory: Vec<String>,
}

impl GameState {
    pub fn new(scenario: &Scenario) -> Self {
        Self {
            current_hp: scenario.initial_hp,
            current_scene: scenario.get_next_scene(scenario.start_scene.clone()),
            inventory: Vec::new(),
        }
    }
    pub fn print_status(&self) -> () {
        println!("Current HP : {}", self.current_hp);
        self.current_scene.print_scene();
    }

    pub fn print_current_scene(&self) -> () {
        self.current_scene.print_scene();
    }

    pub fn print_inventory(&self) -> () {
        for item in self.inventory.iter() {
            println!("- {}", item);
        }
    }

    pub fn is_choice_ok(&self, choice_id: usize) -> bool {
        choice_id < self.current_scene.get_number_choices()
    }

    pub fn change_scene(&mut self, scenario: &Scenario, choice_id: usize) -> () {
        self.current_scene = scenario.get_next_scene(self.current_scene.get_scene_choice(choice_id))
    }

    pub fn has_necessary_item(&self, choice_id: usize) -> bool {
        match self.current_scene.get_scene_item(choice_id) {
            Some(s) => self.inventory.contains(&s),
            None => true,
        }
    }

    pub fn is_ending(&self) -> bool {
        self.current_scene.is_ending()
    }

    pub fn update(&mut self) -> () {
        if self.current_scene.found_items.is_some() {
            self.inventory
                .push(self.current_scene.found_items.clone().unwrap());
        }
        self.current_hp += self.current_scene.get_hp();
    }
}
