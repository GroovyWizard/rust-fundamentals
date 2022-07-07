use std::io::stdin;

fn request_warrior_name() -> String {
    let mut input_name: String = String::new();
    stdin()
        .read_line(&mut input_name)
        .expect("Failed to read line, we are sorry warrior.");

    input_name.trim().to_lowercase()
}

fn validate_marine(name: &&str) -> bool {
    let space_marines_list: [&str; 4] = ["noron", "olaf", "luciun", "nostrum"];
    for marine in &space_marines_list {
        if name == marine {
            return true;
        }
    }

    false
}

fn main() {
    println!("What's your name Warrior?");
    let user_name: String = request_warrior_name();
    let sliced_name: &str = &user_name[..];
    if validate_marine(&sliced_name) {
        return println!("Welcome to the Battleship, {}!", user_name);
    } 

    return println!("Get off Heretic, you are not allowed! General, put {} in the blacklist!", sliced_name);
}
