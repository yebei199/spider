//! Detect Chrome executable path

/// Get the chrome executable path, especially in the nixos
pub fn get_detect_chrome_executable() -> Option<String> {
    if let Ok(path) = std::env::var("CHROME_BIN") {
        return Some(path);
    }
    // Check for NixOS and other paths
    let paths = [
        "/run/current-system/sw/bin/google-chrome-stable",
        "/run/current-system/sw/bin/chromium",
        "/usr/bin/google-chrome-stable",
        "/usr/bin/chromium",
        "/usr/bin/chromium-browser",
        "/usr/bin/google-chrome",
        "/Applications/Google Chrome.app/Contents/MacOS/Google Chrome",
        "/Applications/Chromium.app/Contents/MacOS/Chromium",
    ];
    for path in paths.iter() {
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
    async fn test1() -> () {
        let path =get_detect_chrome_executable();
        if path.is_some() {
            println!("{}", path.unwrap());
        }
        ()
    }

}
