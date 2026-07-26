fn main() {
    #[derive(Debug, Clone, Copy)]
    struct Coord { lat: f64, lon: f64 }
    fn print_coord(c: Coord) { println!("{}, {}", c.lat, c.lon); }
    let home = Coord { lat: 51.5, lon: -0.1 };
    print_coord(home);
    print_coord(home);
}
