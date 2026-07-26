use rand::Rng;

fn main() {
    let secret: u32 = rand::thread_rng().gen_range(1..=100);
    println!("Random secret: {secret}");
}
