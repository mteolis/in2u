use std::collections::HashMap;

use text_io::try_read;

fn main() {
    println!("in2u has started");

    let player_count = get_player_count();

    let players = get_player_names(&player_count);
    println!("Players: {:?}", &players);
    println!("");

    let genders = get_player_genders(&players);
    print_genders(&players, &genders);
    println!("");
}

fn get_player_count() -> i32 {
    print!("Please enter the number of players: ");

    let mut input: Result<i32, _>;
    loop {
        input = try_read!();
        match input {
            Ok(player_count) => {
                println!("Number of players: {}", player_count);
                return player_count;
            }
            Err(_) => {
                println!("Invalid input, please enter an integer.");
                continue;
            }
        }
    }
}

fn get_player_names(player_count: &i32) -> Vec<String> {
    println!("");
    let mut players = Vec::new();

    let mut name: Result<String, _>;
    for i in 0..*player_count {
        print!("Enter player {}'s name: ", i + 1);

        name = try_read!();
        match name {
            Ok(name) => {
                println!("{} added.", &name);
                println!("");
                players.push(name);
                continue;
            }
            Err(_) => {
                println!("Invalid input, please only enter valid strings.");
                println!("");
                continue;
            }
        }
    }

    players
}

fn print_genders(players: &Vec<String>, genders: &HashMap<String, String>) {
    println!("Player genders:");

    for player in players.iter() {
        println!("{}: {:?}", player, genders.get(player).unwrap());
    }
}

fn get_player_genders(players: &Vec<String>) -> HashMap<String, String> {
    let mut player_genders: HashMap<String, String> = HashMap::new();

    let mut selection: Result<i32, _>;
    for player in players.iter() {
        println!("Enter {}'s gender.", &player);

        loop {
            print!("Enter 1 for Male and 2 for Female: ");
            selection = try_read!();
            match selection {
                Ok(selection) => {
                    if selection == 1 {
                        player_genders.insert(player.to_string(), "Male".
                            to_string());
                        println!("{}'s gender assigned to: {}",
                            player, "Male");
                        println!("");
                        break;
                    } else if selection == 2 {
                        player_genders.insert(player.to_string(), "Female".
                            to_string());
                        println!("{}'s gender assigned to: {}",
                            player, "Female");
                        println!("");
                        break;
                    } else {
                        println!("Invalid input, please only enter 1 or 2.");
                        println!("");
                        continue;
                    }
                }
                Err(_) => {
                    println!("Invalid input, please only enter valid strings.");
                    println!("");
                    continue;
                }
            }
        }
    }

    player_genders
}
