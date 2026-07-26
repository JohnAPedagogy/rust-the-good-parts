fn main() {
    struct QueryBuilder { table: String, limit: Option<u32>, filter: Option<String> }
    impl QueryBuilder {
        fn new(table: &str) -> Self {
            QueryBuilder { table: table.to_string(), limit: None, filter: None }
        }
        fn limit(mut self, n: u32) -> Self { self.limit = Some(n); self }
        fn filter(mut self, condition: &str) -> Self {
            self.filter = Some(condition.to_string()); self
        }
        fn build(self) -> String {
            let mut q = format!("SELECT * FROM {}", self.table);
            if let Some(f) = self.filter { q.push_str(&format!(" WHERE {}", f)); }
            if let Some(l) = self.limit { q.push_str(&format!(" LIMIT {}", l)); }
            q
        }
    }
    let query = QueryBuilder::new("users").filter("active = true").limit(10).build();
    println!("{}", query);
}
