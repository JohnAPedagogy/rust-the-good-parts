fn main() {
    mod game2 {
        pub fn play() {
            println!("Playing the game!");
        }
    }
    use game2::play;
    play();
}
