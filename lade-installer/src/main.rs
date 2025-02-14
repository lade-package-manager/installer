use std::{
    fs,
    io::{self, BufRead, Write},
    fs::File,
};

use consts::LADE_VERSION;
use download_urls::LADE_URL;
use paths::{
    lade_bin_path, lade_build_path, lade_cache_path, lade_log_path, lade_package_list_path,
    lade_packages_installed_path,
};
mod check_dependencies;
mod consts;
mod download_file;
mod download_urls;
mod macros;
mod paths;
mod set_env;
mod unzip_file;

#[cfg(unix)]
const TTY_PATH: &str = "/dev/tty";

#[cfg(windows)]
const TTY_PATH: &str = "CON";

fn main() {
    let msg = r#"
\x1b[1mWelcome to the lade installer!\x1b[0m

This program installs lade into your environment and makes it executable.

When you confirm the operation, this program will install lade into your environment.

Are you sure?

y/n> "#
    let tty_first = File::open(TTY_PATH).unwrap();
    let mut read = io::BufReader::new(tty);
    let mut y_or_n = String::new();
    print!("{}", msg);
    io::stdout().flush().unwrap();
    reader.read_line(&mut y_or_n).unwrap();
    
    if y_or_n.trim() != "y"{
        eprintln!(
            "\x1b[31;1m>>> \x1b[1mInstallation canceled\x1b[0m",
            e
        );
        std::process::exit(1);
    }
    info!("Starting installation of \"lade\" (v{})...", LADE_VERSION);
    info!("Checking dependencies...");
    check_dependencies::check_dependencies();
    info!("All dependencies are satisfied.");
    info!("Downloading \"lade\"...");
    #[allow(unused)]
    let mut url = LADE_URL;

    let file = download_file::download_file(url).unwrap_or_else(|e| {
        eprintln!(
            "\x1b[31;1m>>> \x1b[1mERROR: failed to download file: {}\x1b[0m",
            e
        );
        std::process::exit(1);
    });

    info!("Extracting files...");
    let file = unzip_file::extract_zip(file).unwrap_or_else(|e| {
        eprintln!(
            "\x1b[31;1m>>> \x1b[1mERROR: failed to extract file: {}\x1b[0m",
            e
        );
        std::process::exit(1);
    });

    info!("Installing \"lade\" to {}", lade_bin_path().display());

    let dirs = [
        lade_bin_path(),
        lade_build_path(),
        lade_cache_path(),
        lade_log_path(),
        lade_package_list_path(),
        lade_packages_installed_path(),
    ];

    for dir in dirs {
        if !dir.exists() {
            fs::create_dir_all(dir).unwrap_or_else(|e| {
                eprintln!(
                    "\x1b[31;1m>>> \x1b[1mERROR: failed to install lade: {}\x1b[0m",
                    e
                );
                std::process::exit(1);
            });
        }
    }

    fs::rename(
        &file,
        lade_bin_path().join(file.file_name().unwrap().to_str().unwrap()),
    )
    .unwrap_or_else(|e| {
        eprintln!(
            "\x1b[31;1m>>> \x1b[1mERROR: failed to install lade: {}\x1b[0m",
            e
        );
        std::process::exit(1);
    });

    info!("Setting up configurations...");
    println!("Do you want to include lade in your PATH environment variable?");
    print!("[y/N] ");
    io::stdout().flush().unwrap();

    let tty = File::open(TTY_PATH).unwrap();
    let mut reader = io::BufReader::new(tty);
    let mut input = String::new();
    reader.read_line(&mut input).unwrap();

    if input.trim() == "y" || input.trim().is_empty() {
        set_env::add_to_path(lade_bin_path().to_str().unwrap()).unwrap_or_else(|e| {
            eprintln!(
                "\x1b[31;1m>>> \x1b[1mERROR: failed to add path: {}\x1b[0m",
                e
            );
            std::process::exit(1);
        });
    }
    chmod!(lade_bin_path().join("lade"));

    info!(
        "Installation of \"lade\" (v{}) completed successfully!",
        LADE_VERSION
    );
    info!("Run 'lade update; lade --help' to get started.");
}

