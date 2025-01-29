#[macro_export]
macro_rules! info {
    ($($arg:tt)*) => {{
        use std::io::Write;
        print!("\x1b[32;1m>>> \x1b[0m\x1b[1m");
        println!($($arg)*);
        print!("\x1b[0m");
        std::io::stdout().flush().unwrap();
    }};
}
