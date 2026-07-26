fn main() {
    struct ManagedFile { name: String }
    impl Drop for ManagedFile {
        fn drop(&mut self) { println!("Closing: {}", self.name); }
    }
    let _f1 = ManagedFile { name: "config.toml".into() };
    {
        let _f2 = ManagedFile { name: "data.csv".into() };
        println!("Both open");
    }
    println!("Only config remains");
}
