use std::{io::stdin};

struct SpaceMarine {
    name: String,
    faction: String,
}

impl SpaceMarine {
    fn new(name: &str, faction: &str) -> Self {
        Self {
            name: name.to_lowercase(),
            faction: faction.to_string(),
        }
    }
    fn greet_marine(&self) {
        println!("Welcome {} from {}", self.name, self.faction)
    }
}

fn request_warrior_name() -> String {
    let mut input_name: String = String::new();
    stdin()
        .read_line(&mut input_name)
        .expect("Failed to read line, we are sorry warrior.");

    input_name.trim().to_lowercase()
}

fn validate_marine(name: String) {
    let space_marines_list: [SpaceMarine; 4] = [
        SpaceMarine::new("noron", "space wolves"),
        SpaceMarine::new("olaf", "space wolves"),
        SpaceMarine::new("luciun", "black templars"),
        SpaceMarine::new("nostrum", "space marines"),
    ];
    let approved_marine = &space_marines_list.iter().find(|marine| marine.name == name);
   
    match approved_marine {
        Some(marine) => marine.greet_marine(),
        None => {
            println!("Get off Heretic, you are not allowed! General, put {} in the blacklist!", name)
        }
    }
}

fn main() {
    println!("What's your name Warrior?");
    let user_name: String = request_warrior_name();
    validate_marine(user_name);
    
  
    
}
