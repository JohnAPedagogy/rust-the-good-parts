fn main() {
    const MAX_SCORE: u32 = 1_000;
    const BONUS_MULTIPLIER: u32 = 3;
    let player_score: u32 = 750;
    let total = player_score + BONUS_MULTIPLIER * 50;
    println!("Score: {total} (max: {MAX_SCORE})");
}
