fn main() {
    enum Direction { North, _South, _East, _West }
    let heading = Direction::North;
    match heading {
        Direction::North => println!("Heading north"),
        Direction::_South => println!("Heading south"),
        Direction::_East => println!("Heading east"),
        Direction::_West => println!("Heading west"),
    }

    enum GameOutcome { _InProgress, Won(String), _Draw }
    let outcome = GameOutcome::Won("Alice".to_string());
    match outcome {
        GameOutcome::_InProgress => println!("Still playing"),
        GameOutcome::Won(name) => println!("{name} wins!"),
        GameOutcome::_Draw => println!("It's a draw"),
    }
}
