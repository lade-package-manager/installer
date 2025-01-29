use crate::info;
use crate::paths::lade_build_path;
use std::fs::File;
use std::io;
use std::path::{Path, PathBuf};
use zip::read::ZipArchive;

pub fn extract_zip<P: AsRef<Path>>(file_path: P) -> Result<PathBuf, Box<dyn std::error::Error>> {
    let file = File::open(file_path)?;
    let mut archive = ZipArchive::new(file)?;
    let output_dir = lade_build_path();

    for i in 0..archive.len() {
        let mut file = archive.by_index(i)?;
        let out_path = output_dir.join(file.name());

        if file.is_dir() {
            std::fs::create_dir_all(&out_path)?;
        } else {
            if let Some(parent) = out_path.parent() {
                std::fs::create_dir_all(parent)?;
            }

            let mut outfile = File::create(&out_path)?;
            io::copy(&mut file, &mut outfile)?;
        }

        info!("Extracted {}", out_path.display());
    }

    #[allow(unused)]
    let mut out = PathBuf::new();
    if !cfg!(target_os = "windows") {
        out = lade_build_path().join("lade");
    } else {
        out = lade_build_path().join("lade.exe");
    }

    Ok(out)
}
