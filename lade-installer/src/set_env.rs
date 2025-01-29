use std::io::{self, Write, BufRead, BufReader};
use std::fs::OpenOptions;

#[cfg(target_os = "windows")]
use winreg::enums::*;
#[cfg(target_os = "windows")]
use winreg::RegKey;

#[cfg(not(target_os = "windows"))]
pub fn add_to_path(new_path: &str) -> io::Result<()> {
    let home_dir = dirs_next::home_dir().ok_or(io::Error::new(io::ErrorKind::NotFound, "ホームディレクトリが見つかりません"))?;
    let bashrc_path = home_dir.join(".bashrc");

    let file = OpenOptions::new().read(true).open(&bashrc_path)?;
    let reader = BufReader::new(file);
    let already_in_path = reader.lines().any(|line| {
        let line = line.unwrap_or_default();
        line.contains(&format!("export PATH=")) && line.contains(new_path)
    });

    if already_in_path {
        return Ok(());
    }

    let mut file = OpenOptions::new().append(true).open(bashrc_path)?;
    writeln!(file, "export PATH=\"$PATH:{}\"", new_path)?;

    println!("Added to PATH enviroment variable");
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