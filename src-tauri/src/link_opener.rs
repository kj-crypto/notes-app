#[tauri::command]
pub fn open_url(
    url: String,
    browser: Option<String>,
    incognito: Option<bool>,
) -> Result<(), String> {
    let incognito = incognito.unwrap_or(false);

    #[cfg(target_os = "windows")]
    {
        match browser.as_deref() {
            Some("firefox") => {
                let mut cmd = std::process::Command::new("firefox");
                if incognito {
                    cmd.arg("--private-window");
                }
                cmd.arg(&url).spawn().map_err(|e| e.to_string())?;
            }
            Some("chrome") | Some("google-chrome") => {
                let mut cmd = std::process::Command::new("chrome");
                if browser.as_deref() == Some("google-chrome") {
                    cmd = std::process::Command::new("google-chrome");
                }
                if incognito {
                    cmd.arg("--incognito");
                }
                cmd.arg(&url).spawn().map_err(|e| e.to_string())?;
            }
            Some("chromium") => {
                let mut cmd = std::process::Command::new("chromium");
                if incognito {
                    cmd.arg("--incognito");
                }
                cmd.arg(&url).spawn().map_err(|e| e.to_string())?;
            }
            Some("brave") | Some("brave-browser") => {
                let mut cmd = std::process::Command::new("brave");
                if browser.as_deref() == Some("brave-browser") {
                    cmd = std::process::Command::new("brave-browser");
                }
                if incognito {
                    cmd.arg("--incognito");
                }
                cmd.arg(&url).spawn().map_err(|e| e.to_string())?;
            }
            _ => {
                std::process::Command::new("cmd")
                    .args(&["/C", "start", "", &url])
                    .spawn()
                    .map_err(|e| e.to_string())?;
            }
        }
    }
    #[cfg(target_os = "macos")]
    {
        match browser.as_deref() {
            Some("firefox") => {
                let mut cmd = std::process::Command::new("open");
                cmd.args(&["-a", "Firefox"]);
                if incognito {
                    cmd.arg("--args").arg("--private-window");
                }
                cmd.arg(&url).spawn().map_err(|e| e.to_string())?;
            }
            Some("chrome") | Some("google-chrome") => {
                let mut cmd = std::process::Command::new("open");
                cmd.args(&["-a", "Google Chrome"]);
                if incognito {
                    cmd.arg("--args").arg("--incognito");
                }
                cmd.arg(&url).spawn().map_err(|e| e.to_string())?;
            }
            Some("chromium") => {
                let mut cmd = std::process::Command::new("open");
                cmd.args(&["-a", "Chromium"]);
                if incognito {
                    cmd.arg("--args").arg("--incognito");
                }
                cmd.arg(&url).spawn().map_err(|e| e.to_string())?;
            }
            Some("brave") | Some("brave-browser") => {
                let mut cmd = std::process::Command::new("open");
                cmd.args(&["-a", "Brave Browser"]);
                if incognito {
                    cmd.arg("--args").arg("--incognito");
                }
                cmd.arg(&url).spawn().map_err(|e| e.to_string())?;
            }
            _ => {
                std::process::Command::new("open")
                    .arg(&url)
                    .spawn()
                    .map_err(|e| e.to_string())?;
            }
        }
    }
    #[cfg(target_os = "linux")]
    {
        match browser.as_deref() {
            Some("firefox") => {
                let mut cmd = std::process::Command::new("firefox");
                if incognito {
                    cmd.arg("--private-window");
                }
                cmd.arg(&url).spawn().map_err(|e| e.to_string())?;
            }
            Some("chromium") => {
                let mut cmd = std::process::Command::new("chromium");
                if incognito {
                    cmd.arg("--incognito");
                }
                cmd.arg(&url).spawn().map_err(|e| e.to_string())?;
            }
            Some("chrome") | Some("google-chrome") => {
                let mut cmd = std::process::Command::new("google-chrome");
                if incognito {
                    cmd.arg("--incognito");
                }
                cmd.arg(&url).spawn().map_err(|e| e.to_string())?;
            }
            Some("brave") | Some("brave-browser") => {
                let mut cmd = std::process::Command::new("brave-browser");
                if browser.as_deref() == Some("brave") {
                    cmd = std::process::Command::new("brave");
                }
                if incognito {
                    cmd.arg("--incognito");
                }
                cmd.arg(&url).spawn().map_err(|e| e.to_string())?;
            }
            _ => {
                std::process::Command::new("xdg-open")
                    .arg(&url)
                    .spawn()
                    .map_err(|e| e.to_string())?;
            }
        }
    }
    Ok(())
}
