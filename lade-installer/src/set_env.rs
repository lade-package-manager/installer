use std::io::{self, Write, BufRead, BufReader};
use std::fs::OpenOptions;

#[cfg(target_os = "windows")]
use winreg::enums::*;
#[cfg(target_os = "windows")]
use winreg::RegKey;

#[cfg(not(target_os = "windows"))]
pub fn add_to_path(new_path: &str) -> io::Result<()> {
    use crate::info;

    let home_dir = dirs_next::home_dir().ok_or(io::Error::new(
        io::ErrorKind::NotFound,
        "Home directory not found",
    ))?;

    let shell_files = vec![
        (home_dir.join(".bashrc"), format!("export PATH=\"$PATH:{}\"", new_path)),
        (home_dir.join(".zshrc"), format!("export PATH=\"$PATH:{}\"", new_path)),
        (home_dir.join(".config/fish/config.fish"), format!("set -x PATH {} $PATH", new_path)),
    ];

    for (rc_file, export_cmd) in shell_files {
        if rc_file.exists() {
            let file = OpenOptions::new().read(true).open(&rc_file)?;
            let reader = BufReader::new(file);
            let already_in_path = reader.lines().any(|line| {
                let line = line.unwrap_or_default();
                line.contains(&export_cmd)
            });

            if !already_in_path {
                let mut file = OpenOptions::new().append(true).open(&rc_file)?;
                writeln!(file, "{}", export_cmd)?;
                info!("Added {} to {}", new_path, rc_file.display());
            }
        }
    }

    Ok(())
}


#[cfg(target_os = "windows")]
pub fn add_to_path(new_path: &str) -> io::Result<()> {
    use crate::info;

    let hkcu = RegKey::predef(HKEY_CURRENT_USER);
    let env_key = hkcu.open_subkey_with_flags("Environment", KEY_READ | KEY_WRITE)?;

    // 現在のPATHを取得
    let current_path: String = env_key.get_value("Path").unwrap_or_default();

    // すでに登録されていないか確認
    if current_path.split(';').any(|p| p == new_path) {
        return Ok(());
    }

    // 新しいPATHを追加
    let updated_path = if current_path.is_empty() {
        new_path.to_string()
    } else {
        format!("{};{}", current_path, new_path)
    };
    env_key.set_value("Path", &updated_path)?;

    info!("Added to Path enviroment variable");
    Ok(())
}