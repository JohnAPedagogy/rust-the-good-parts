fn main() {
    struct Envelope { to: String, subject: String, body: String }
    let env = Envelope {
        to: String::from("alice@example.com"),
        subject: String::from("Hello"),
        body: String::from("How are you?"),
    };
    let _to = env.to;
    let _subject = env.subject;
    println!("{}", env.body);
}
