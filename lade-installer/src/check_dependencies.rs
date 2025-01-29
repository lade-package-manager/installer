use std::process::{Command, Stdio};

pub fn check_dependencies() {
    let depends = ["cargo", "rustc", "sh"];

    for depen in depends {
        if !cfg!(target_os = "windows") {
            let status = Command::new("which")
                .arg(depen)
                .stdout(Stdio::null())
                .stderr(Stdio::null())
                .status()
                .unwrap_or_else(|e| {
                    eprintln!(
                        "\x1b[31;1m>>> \x1b[1mFailed to check dependencies: {}\x1b[0m",
                        e
                    );
                    std::process::exit(1);
                });

            if !status.success() {
                eprintln!(
                    "\x1b[31;1m>>> \x1b[1mERROR: Missing required dependency: {}\x1b[0m",
                    depen
                );
                eprintln!("\x1b[31;1m>>> \x1b[1mPlease install it manually and re-run the installer.\x1b[0m");
                std::process::exit(1);
            }
        } else {
            let status = Command::new("where")
                .arg(depen)
                .stdout(Stdio::null())
                .stderr(Stdio::null())
                .status()
                .unwrap_or_else(|e| {
                    eprintln!(
                        "\x1b[31;1m>>> \x1b[1mFailed to check dependencies: {}\x1b[0m",
                        e
                    );
                    std::process::exit(1);
                });

            if !status.success() {
                eprintln!(
                    "\x1b[31;1m>>> \x1b[1mERROR: Missing required dependency: {}\x1b[0m",
                    depen
                );
                eprintln!("\x1b[31;1m>>> \x1b[1mPlease install it manually and re-run the installer.\x1b[0m");
                std::process::exit(1);
            }
        }
    }
}
