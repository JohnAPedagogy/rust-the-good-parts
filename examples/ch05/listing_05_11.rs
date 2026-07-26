fn main() {
    struct Metres(f64);
    struct MetresPerSecond(f64);
    struct Seconds(f64);
    fn time_to_travel(distance: Metres, speed: MetresPerSecond) -> Seconds {
        Seconds(distance.0 / speed.0)
    }
    let d = Metres(100.0);
    let s = MetresPerSecond(10.0);
    let t = time_to_travel(d, s);
    println!("Time: {} seconds", t.0);
}
