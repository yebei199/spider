//! Detect Chrome executable path

/// Get the chrome executable path.
pub fn get_detect_chrome_executable() -> Option<String> {
    // 1. Check CHROME_BIN environment variable
    if let Ok(path) = std::env::var("CHROME_BIN") {
        return Some(path);
    }

    // 2. Check standard executables in PATH using `which`
    let chrome_names = [
        "google-chrome-stable",
        "chromium",
        "google-chrome",
        "chrome",
        "chromium-browser",
    ];

    for name in &chrome_names {
        if let Ok(output) = std::process::Command::new("which")
            .arg(name)
            .output()
        {
            if output.status.success() {
                let path = String::from_utf8_lossy(&output.stdout).trim().to_string();
                if !path.is_empty() {
                    return Some(path);
                }
            }
        }
    }

    // 3. Check hardcoded fallback paths (NixOS, macOS, etc.)
    let fallback_paths = [
        "/run/current-system/sw/bin/google-chrome-stable",
        "/run/current-system/sw/bin/chromium",
        "/usr/bin/google-chrome-stable",
        "/usr/bin/chromium",
        "/usr/bin/chromium-browser",
        "/usr/bin/google-chrome",
        "/Applications/Google Chrome.app/Contents/MacOS/Google Chrome",
        "/Applications/Chromium.app/Contents/MacOS/Chromium",
    ];

    for path in fallback_paths.iter() {
        if std::path::Path::new(path).exists() {
            return Some(path.to_string());
        }
    }

    None
}

#[cfg(test)]
mod test {
    use super::*;

    #[tokio::test]
    async fn test_chrome_path() -> () {
        let path = get_detect_chrome_executable();
        assert!(path.is_some(), "Chrome executable should be found");
        dbg!( path);

    }
}
