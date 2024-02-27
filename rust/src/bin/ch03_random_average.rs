use search::ch03::game;
use search::ch03::random_action;

fn main() {
    //
    let num_games = 100;
    let average =
        game::average(Box::new(random_action::random_action), num_games, 10);
    println!("avergae {average} of random action over num_games {num_games}");
}
