fn main() {
    let is_raining = true;
    let temperature = 22;
    let nice_day = !is_raining && temperature > 18;
    println!("Nice day? {nice_day}");
}
