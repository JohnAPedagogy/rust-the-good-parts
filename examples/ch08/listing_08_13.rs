fn main() {
    #[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
    struct Version { major: u32, minor: u32, patch: u32 }
    let mut versions = vec![
        Version { major: 1, minor: 2, patch: 3 },
        Version { major: 0, minor: 9, patch: 0 },
        Version { major: 2, minor: 0, patch: 0 },
    ];
    versions.sort();
    for v in &versions { println!("{}.{}.{}", v.major, v.minor, v.patch); }
}
