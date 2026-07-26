use std::fmt;

fn main() {
    #[derive(Debug)]
    enum DbError {
        Connection(String),
        NotFound(u32),
    }
    impl fmt::Display for DbError {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            match self {
                DbError::Connection(e) => write!(f, "Connection: {e}"),
                DbError::NotFound(id) => write!(f, "Not found: {id}"),
            }
        }
    }
    impl std::error::Error for DbError {}
    fn fetch_user_score(_user_id: u32) -> Result<String, DbError> {
        Err(DbError::NotFound(_user_id))
    }
    #[derive(Debug)]
    enum ServiceError {
        Database(String),
        BadScore(String),
    }
    impl fmt::Display for ServiceError {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            match self {
                ServiceError::Database(e) => write!(f, "DB: {e}"),
                ServiceError::BadScore(e) => write!(f, "Score: {e}"),
            }
        }
    }
    impl std::error::Error for ServiceError {}
    impl From<DbError> for ServiceError {
        fn from(e: DbError) -> Self { ServiceError::Database(e.to_string()) }
    }
    fn get_user_score(user_id: u32) -> Result<i32, ServiceError> {
        let raw = fetch_user_score(user_id)?;
        raw.trim().parse::<i32>().map_err(|e| ServiceError::BadScore(e.to_string()))
    }
    match get_user_score(42) {
        Ok(score) => println!("Score: {score}"),
        Err(ServiceError::Database(e)) => println!("DB error: {e}"),
        Err(ServiceError::BadScore(e)) => println!("Parse error: {e}"),
    }
}
