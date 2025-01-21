pub fn get_database_url() -> String {
    let env_val_result = std::env::var("DATABASE_URL");
    if env_val_result.is_err() {
        panic!("DATABASE_URL environment variable is not set.");
    }
    let env_val = env_val_result.unwrap();
    if !env_val.starts_with("postgresql://") {
        panic!("DATABASE_URL must start with 'postgresql://'");
    }
    env_val
}

/// Example usage
pub fn main() {
    std::env::set_var("DATABASE_URL", "postgresql://localhost");

    let db_url = get_database_url();
    println!("Database URL: {}", db_url);

    std::env::remove_var("DATABASE_URL"); // Missing variable scenario
    let _db_url = get_database_url();

    std::env::set_var("DATABASE_URL", "mysql://localhost"); // Invalid prefix scenario
    let _db_url = get_database_url();
}

