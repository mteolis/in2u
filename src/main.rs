use text_io::try_read;

fn main() {
    println!("in2u has started");

    let player_count = get_player_count();
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
