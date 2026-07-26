fn main() {
    mod game {
        pub fn play() {
            let secret = generate_secret();
            println!("Secret: {secret}");
        }
        fn generate_secret() -> u32 {
            42
        }
    }
    game::play();
}
