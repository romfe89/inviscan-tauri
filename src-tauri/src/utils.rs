use std::collections::HashSet;

pub fn log_info(msg: &str) {
    println!("\x1b[34m[INFO]\x1b[0m {}", msg);
}

pub fn log_success(msg: &str) {
    println!("\x1b[32m[SUCCESS]\x1b[0m {}", msg);
}

pub fn log_warning(msg: &str) {
    println!("\x1b[33m[WARNING]\x1b[0m {}", msg);
}

pub fn log_error(msg: &str) {
    println!("\x1b[31m[ERROR]\x1b[0m {}", msg);
}

pub fn remove_duplicates(mut items: Vec<String>) -> Vec<String> {
    let set: HashSet<_> = items.drain(..).collect();
    set.into_iter().collect()
}

pub fn log_debug(msg: &str) {
    println!("\x1b[38;5;208m[DEBUG]\x1b[0m {}", msg);
}