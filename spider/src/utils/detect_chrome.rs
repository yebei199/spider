//! Detect Chrome executable path

/// Get the chrome executable path.
pub fn get_detect_chrome_executable() -> Option<String> {
    // 1. Check CHROME_BIN environment variable
    if let Ok(path) = std::env::var("CHROME_BIN") {
        return Some(path);
    }

    // 2. Check standard executables in PATH using the `which` crate
    let chrome_names = [
        "google-chrome-stable",
        "chromium",
        "google-chrome",
        "chrome",
        "chromium-browser",
        "google-chrome-beta",
        "google-chrome-unstable",
    ];

    for name in &chrome_names {
        if let Ok(path) = which::which(name) {
            return Some(path.to_string_lossy().to_string());
        }
    }

    // 3. Check common paths in HOME directory
    if let Some(home) = home::home_dir() {
        let paths = [
            home.join("Applications/Google Chrome.app/Contents/MacOS/Google Chrome"),
            home.join(".local/bin/google-chrome-stable"),
            home.join(".local/bin/chromium"),
            home.join(".local/bin/chrome"),
            home.join("bin/google-chrome-stable"),
            home.join("bin/chromium"),
            home.join("bin/chrome"),
        ];
        for path in paths.iter() {
            if path.exists() {
                return Some(path.to_string_lossy().to_string());
            }
        }
    }

    // 4. Check hardcoded fallback paths (NixOS, MacOS, Linux, Windows)
    let fallback_paths = [
        "/run/current-system/sw/bin/google-chrome-stable",
        "/run/current-system/sw/bin/chromium",
        "/usr/bin/google-chrome-stable",
        "/usr/bin/chromium",
        "/usr/bin/chromium-browser",
        "/usr/bin/google-chrome",
        "/Applications/Google Chrome.app/Contents/MacOS/Google Chrome",
        "/Applications/Chromium.app/Contents/MacOS/Chromium",
        "C:\\Program Files\\Google\\Chrome\\Application\\chrome.exe",
        "C:\\Program Files (x86)\\Google\\Chrome\\Application\\chrome.exe",
        "C:\\Program Files\\Chromium\\Application\\chrome.exe",
    ];

    for path in fallback_paths.iter() {
        let p = std::path::Path::new(path);
        if p.exists() {
            return Some(p.to_string_lossy().to_string());
        }
    }

    None
}

#[cfg(test)]
mod test {
    use super::*;

    #[tokio::test]
    async fn test_detect_chrome() {
        let path = get_detect_chrome_executable();
        assert!(path.is_some());
        println!("Chrome path: {}", path.unwrap());
    }
}
