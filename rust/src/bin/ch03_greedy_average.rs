use search::ch03::game;
use search::ch03::greedy;

fn main() {
    //
    let num_games = 100;
    let average = game::average(greedy::greedy_action, num_games, 10);
    println!("avergae {average} of random action over num_games {num_games}");
}
