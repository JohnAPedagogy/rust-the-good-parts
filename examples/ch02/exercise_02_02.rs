fn main() {
    let temperature: f64 = 20.0;
    let category = match temperature {
        t if t < 0.0 => "freezing",
        t if t <= 15.0 => "cold",
        t if t <= 25.0 => "comfortable",
        _ => "hot",
    };
    println!("{temperature}°C is {category}");
}
