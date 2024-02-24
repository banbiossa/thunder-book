use game::ch03::maze_state::NumberCollectingGame;

fn main() {
    let state = NumberCollectingGame::new(0);
    println!("state is done? {}", state.is_done());
    println!("points : {:?}", state.points);
    println!("hello from plays");

    println!("character {:?}", state.character);
}
