use text_io::try_read;

fn main() {
    println!("in2u has started");

    let player_count = get_player_count();
    let players = get_player_names(&player_count);
    println!("Players: {:?}", &players);
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
    let mut players = Vec::new();

    let mut name: Result<String, _>;
    for i in 0..*player_count {
        print!("Enter player {}'s name: ", i + 1);

        name = try_read!();
        match name {
            Ok(name) => {
                println!("{} added.", &name);
                players.push(name);
                continue;
            }
            Err(_) => {
                println!("Invalid input, please only enter valid strings.");
                continue;
            }
        }
    }

    players
}
