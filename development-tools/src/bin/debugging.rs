use env_logger::{Builder, Target};

fn execute_query(query: &str) {
    log::debug!("Executing query: {}", query);
}

fn log_debug_message_to_the_console() {
    env_logger::init();

    execute_query("DROP TABLE students");
}

fn execute_query_2(_query: &str) -> Result<(), &'static str> {
    Err("I'm afraid I can't do that")
}

fn log_error_message_to_the_console() {
    env_logger::init();

    let response = execute_query_2("DROP TABLE students");
    if let Err(err) = response {
        log::error!("Failed to execute query: {}", err);
    }
}

fn log_to_stdout() {
    Builder::new().target(Target::Stdout).init();

    log::error!("This error has been printed to Stdout");
}

fn main() {
    // log_debug_message_to_the_console();
    // log_error_message_to_the_console();
    log_to_stdout();
}
