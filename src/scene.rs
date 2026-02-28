use super::errors::{GameError, ParseError};
use serde::Deserialize;
use std::{fs::File, io::Read};

#[derive(Deserialize, Clone)]
struct Choice {
    label: String,
    next: String,
    required_item: Option<String>,
}

impl Choice {
    pub fn print_choice(&self) -> () {
        print!("- {} ", self.label);
        if self.required_item.is_some() {
            print!("(Required : {})", self.required_item.clone().unwrap());
        }
        println!()
    }
}

#[derive(Deserialize, Clone)]
pub struct Scene {
    id: String,
    title: String,
    text: String,
    //#[serde(default)]
    hp_delta: Option<i32>,
    pub found_items: Option<String>,
    ending: Option<String>,
    choices: Option<Vec<Choice>>,
}

impl Scene {
    pub fn print_scene(&self) -> () {
        if self.ending.is_some() {
            println!("Ending {}", self.ending.clone().unwrap())
        }
        println!("title : {}", self.title);
        println!("text : {}", self.text);
        if self.hp_delta.is_some() {
            println!(
                "{} {} hp",
                if self.hp_delta.unwrap() < 0 {
                    "Lost"
                } else {
                    "Gained"
                },
                self.hp_delta.unwrap()
            )
        }

        if self.found_items.is_some() {
            println!("Found {}", self.found_items.clone().unwrap())
        }

        for choice in self.choices.clone().unwrap_or_default().iter() {
            choice.print_choice();
        }
    }

    pub fn get_number_choices(&self) -> usize {
        if self.choices.is_none() {
            return 0;
        }
        return self.choices.clone().unwrap().len();
    }

    pub fn get_scene_choice(&self, choice_id: usize) -> String {
        self.choices.clone().unwrap()[choice_id].next.clone()
    }

    pub fn get_scene_item(&self, choice_id: usize) -> Option<String> {
        self.choices.clone().unwrap()[choice_id]
            .required_item
            .clone()
    }

    pub fn get_hp(&self) -> i32 {
        self.hp_delta.unwrap_or_default()
    }

    pub fn is_ending(&self) -> bool {
        self.ending.is_some()
    }
}

#[derive(Deserialize)]
pub struct Scenario {
    pub start_scene: String,
    pub initial_hp: i32,
    scenes: Vec<Scene>,
}

impl Scenario {
    pub fn parse_scene(filename: &str) -> Result<Self, ParseError> {
        let mut file = File::open(filename).unwrap();
        let mut content: String = String::new();
        file.read_to_string(&mut content).unwrap();
        let cfg: Scenario = serde_yaml::from_str(&content).unwrap();
        Ok(cfg)
    }

    pub fn validate_scenario(&self) -> Result<bool, GameError> {
        let scenes_id: Vec<String> = self.scenes.iter().map(|s| s.id.clone()).collect();
        if scenes_id.iter().find(|s| **s != self.start_scene).is_none() {
            return Err(GameError::InvalidScene);
        }

        for i in 0..scenes_id.len() - 1 {
            for j in i + 1..scenes_id.len() {
                if scenes_id[i] == scenes_id[j] {
                    return Err(GameError::InvalidScene);
                }
            }
        }

        for scene in self.scenes.iter() {
            if scene.choices.is_some() {
                for choice in scene.choices.clone().unwrap().iter() {
                    if !scenes_id.contains(&choice.next) {
                        return Err(GameError::InvalidScene);
                    }
                }
            }
        }
        Ok(true)
    }

    pub fn get_next_scene(&self, scene_name: String) -> Scene {
        let new_scene: &Scene = self.scenes.iter().find(|s| *s.id == scene_name).unwrap();
        new_scene.clone()
    }
}
