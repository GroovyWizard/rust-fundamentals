use std::io::stdin;

#[derive(Debug)]
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

fn validate_marine(space_marines_list: &mut Vec<SpaceMarine>, name: String) -> bool {
    let approved_marine = space_marines_list.iter().find(|marine| marine.name == name);

    match approved_marine {
        Some(marine) => {
            marine.greet_marine();
            println!("List of marines:");
            println!("{:#?}", space_marines_list);
            return true;
        }
        None => {
            if name.is_empty() {
                println!(
                    "Get off Heretic, you are not allowed! General, he refuses to say his name!",
                );
                println!("List of marines:");
                println!("{:#?}", space_marines_list);
                return false;
            } else {
                println!("{} is not on the marine list, but he looks rather strong and i want him in the crew, welcome {}" , name, name);
                space_marines_list.push(SpaceMarine::new(&name, "space marine"));
                println!("List of marines:");
                println!("{:#?}", space_marines_list);

                return true;
            }
        }
    }
}

fn main() {
    let mut space_marines_list: Vec<SpaceMarine> = vec![
        SpaceMarine::new("noron", "space wolves"),
        SpaceMarine::new("olaf", "space wolves"),
        SpaceMarine::new("luciun", "black templars"),
        SpaceMarine::new("nostrum", "space marines"),
    ];

    let mut continue_asking: bool = true;
    while continue_asking {
        println!("What's your name Warrior?");
        let user_name: String = request_warrior_name();
        continue_asking = validate_marine(&mut space_marines_list, user_name);
    }
}
