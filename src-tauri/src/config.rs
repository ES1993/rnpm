use std::{path::PathBuf, str::FromStr};

use semver::Version;

pub struct Config {
    pub node_url: String,
    pub node_dir: PathBuf,
    pub platform: String,
    pub arch: String,
}

impl Config {
    pub fn new() -> Self {
        Self {
            node_url: "https://nodejs.org/dist".to_string(),
            node_dir: dirs::home_dir().unwrap().join(".rnpm"),
            platform: platform(),
            arch: arch(),
        }
    }

    pub fn filename(&self, version: &str) -> String {
        let mut arch = self.arch.clone();
        if let Ok(version) = Version::from_str(version.trim_start_matches('v')) {
            if version.major < 16 {
                arch = "x64".to_string();
            }
        }

        #[cfg(unix)]
        let suffix = "tar.xz";
        #[cfg(windows)]
        let suffix = "zip";

        format!(
            "node-{version}-{platform}-{arch}.{suffix}",
            version = version,
            platform = self.platform,
            arch = arch,
            suffix = suffix,
        )
    }

    pub fn download_url(&self, version: &str) -> String {
        format!(
            "{url}/{version}/{filename}",
            url = self.node_url,
            version = version,
            filename = self.filename(version)
        )
    }

    pub fn list_url(&self) -> String {
        format!("{url}/index.json", url = self.node_url)
    }
}

fn platform() -> String {
    #[cfg(target_os = "macos")]
    {
        "darwin".to_string()
    }

    #[cfg(target_os = "linux")]
    {
        "linux".to_string()
    }

    #[cfg(target_os = "windows")]
    {
        "win".to_string()
    }

    #[cfg(not(any(target_os = "windows", target_os = "macos", target_os = "linux")))]
    {
        "unknown".to_string()
    }
}

fn arch() -> String {
    #[cfg(all(
        target_pointer_width = "32",
        any(target_arch = "arm", target_arch = "aarch64")
    ))]
    {
        "armv7l".to_string()
    }

    #[cfg(all(
        target_pointer_width = "32",
        not(any(target_arch = "arm", target_arch = "aarch64"))
    ))]
    {
        "x86".to_string()
    }

    #[cfg(all(
        target_pointer_width = "64",
        any(target_arch = "arm", target_arch = "aarch64")
    ))]
    {
        "arm64".to_string()
    }

    #[cfg(all(
        target_pointer_width = "64",
        not(any(target_arch = "arm", target_arch = "aarch64"))
    ))]
    {
        "x64".to_string()
    }
}
