fn main() {
    #[derive(Debug, Clone, PartialEq)]
    enum GameState { Playing, Won(usize), Draw }
    #[derive(Debug, Clone)]
    struct Player { name: String, score: u32 }
    type Board = [[Option<usize>; 3]; 3];
    struct Game { board: Board, players: [Player; 2], state: GameState }
    impl Game {
        fn new(player1: &str, player2: &str) -> Self {
            Game {
                board: [[None; 3]; 3],
                players: [
                    Player { name: player1.to_string(), score: 0 },
                    Player { name: player2.to_string(), score: 0 },
                ],
                state: GameState::Playing,
            }
        }
    }
    let game = Game::new("Alice", "Bob");
    println!("{:?}", game.state);
}
