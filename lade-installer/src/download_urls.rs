#[cfg(target_os = "linux")]
pub const LADE_URL: &str =
    "https://github.com/lade-package-manager/lade/releases/download/0.1/lade-linux-build.zip";

#[cfg(target_os = "macos")]
pub const LADE_URL: &str =
    "https://github.com/lade-package-manager/lade/releases/download/0.1/lade-macos-build.zip";

#[cfg(target_os = "windows")]
pub const LADE_URL: &str =
    "https://github.com/lade-package-manager/lade/releases/download/0.1/lade-windows-build.zip";

#[cfg(target_os = "windows")]
pub const ZIP_NAME: &str = "lade-windows-build.zip";
#[cfg(target_os = "macos")]
pub const ZIP_NAME: &str = "lade-macos-build.zip";
#[cfg(target_os = "linux")]
pub const ZIP_NAME: &str = "lade-linux-build.zip";
