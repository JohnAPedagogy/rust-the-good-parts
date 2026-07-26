use std::cell::RefCell;
use std::rc::Rc;

fn main() {
    struct GameScore { player: String, score: i32 }
    let score = Rc::new(RefCell::new(GameScore {
        player: "Alice".to_string(),
        score: 0,
    }));
    let score2 = Rc::clone(&score);
    score2.borrow_mut().score += 10;
    println!("{}: {}", score.borrow().player, score.borrow().score);
}
