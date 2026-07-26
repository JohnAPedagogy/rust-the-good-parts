fn main() {
    fn parse_manual(s: &str) -> Result<i32, String> {
        let n = match s.parse::<i32>() {
            Ok(v) => v,
            Err(e) => return Err(format!("{e}")),
        };
        Ok(n * 2)
    }
    fn parse_question(s: &str) -> Result<i32, String> {
        let n = s.parse::<i32>().map_err(|e| format!("{e}"))?;
        Ok(n * 2)
    }
    println!("manual: {:?}", parse_manual("21"));
    println!("question: {:?}", parse_question("21"));
}
