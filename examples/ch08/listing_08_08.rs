use std::fmt;

fn main() {
    struct StringList(Vec<String>);
    impl fmt::Display for StringList {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            write!(f, "[{}]", self.0.join(", "))
        }
    }
    let list = StringList(vec!["alpha".into(), "beta".into(), "gamma".into()]);
    println!("{}", list);
}
