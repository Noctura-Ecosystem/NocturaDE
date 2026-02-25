pub fn log(msg: &str, level: i8) {
    match level {
        0 => println!("[INFO] {}", msg),
        1 => println!("[WARN] {}", msg),
        2 => eprintln!("[ERROR] {}", msg),
        _ => println!("[LOG] {}", msg),
    }
}