use serde::Deserialize;
use std::path::PathBuf;

#[derive(Deserialize)]
struct Release {
    tag_name: String,
    assets: Vec<Asset>,
}

#[derive(Deserialize)]
struct Asset {
    name: String,
    browser_download_url: String,
}

#[derive(Debug, PartialEq, Eq)]
struct Version {
    major: u32,
    minor: u32,
    patch: u32,
}

impl Version {
    fn from_str(s: &str) -> Option<Self> {
        let s = s.strip_prefix('v').unwrap_or(s);
        let parts: Vec<&str> = s.split('.').collect();
        if parts.len() != 3 {
            return None;
        }
        Some(Version {
            major: parts[0].parse().ok()?,
            minor: parts[1].parse().ok()?,
            patch: parts[2].parse().ok()?,
        })
    }
}

impl PartialOrd for Version {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Version {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.major
            .cmp(&other.major)
            .then_with(|| self.minor.cmp(&other.minor))
            .then_with(|| self.patch.cmp(&other.patch))
    }
}

pub fn check_for_updates() {
    let current_ver = Version::from_str(env!("CARGO_PKG_VERSION"));
    if current_ver.is_none() {
        return;
    }
    let current_ver = current_ver.unwrap();

    let response = match ureq::get("https://api.github.com/repos/ZFordDev/MathPlus/releases/latest")
        .set("User-Agent", "MathPlus")
        .call()
    {
        Ok(r) => r,
        Err(_) => return,
    };

    let body = match response.into_string() {
        Ok(b) => b,
        Err(_) => return,
    };

    let release: Release = match serde_json::from_str(&body) {
        Ok(r) => r,
        Err(_) => return,
    };

    let latest_ver = match Version::from_str(&release.tag_name) {
        Some(v) => v,
        None => return,
    };

    if latest_ver <= current_ver {
        return;
    }

    let asset = match release.assets.iter().find(|a| a.name == "MathPlus.exe") {
        Some(a) => a,
        None => return,
    };

    let temp_dir = std::env::temp_dir();
    let download_path: PathBuf = temp_dir.join(&asset.name);

    match ureq::get(&asset.browser_download_url)
        .set("User-Agent", "MathPlus")
        .call()
    {
        Ok(response) => {
            let mut reader = response.into_reader();
            let mut file = match std::fs::File::create(&download_path) {
                Ok(f) => f,
                Err(_) => return,
            };
            if std::io::copy(&mut reader, &mut file).is_ok() {
                println!(
                    "A new version ({}) is available. Downloaded to: {}",
                    release.tag_name,
                    download_path.display()
                );
            }
        }
        Err(_) => {}
    }
}
