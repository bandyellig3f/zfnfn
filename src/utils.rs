use std::fs;

pub fn log_message(message: &str) {
    let log_file = "unlock_log.txt";
    let mut file = fs::OpenOptions::new()
        .create(true)
        .append(true)
        .open(log_file)
        .expect("Unable to open log file");
    
    writeln!(file, "{}", message).expect("Unable to write to log file");
}