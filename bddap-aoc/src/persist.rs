//! Keeps track of session tokens, caches input files.

use std::{collections::HashMap, str::FromStr};

const APP_NAME: &str = "bddap-aoc";
pub const DEFAULT_PROFILE: &str = "default";

#[derive(serde::Deserialize, serde::Serialize, Clone)]
struct ProfileSettings {
    session_token: String,
}

#[derive(serde::Serialize, PartialEq, Eq, Hash)]
struct FileSafeString(String);

impl FromStr for FileSafeString {
    type Err = &'static str;

    fn from_str(name: &str) -> Result<Self, Self::Err> {
        let err = "Profile name must be filename-safe";
        if !name
            .chars()
            .all(|c| c.is_ascii_alphanumeric() || c == '_' || c == '-' || c == ' ')
        {
            return Err(err);
        }
        if name.starts_with('-') || name.starts_with(' ') {
            return Err(err);
        }
        Ok(Self(name.to_owned()))
    }
}

impl<'de> serde::Deserialize<'de> for FileSafeString {
    fn deserialize<D>(deserializer: D) -> Result<FileSafeString, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        s.parse().map_err(serde::de::Error::custom)
    }
}

#[derive(serde::Deserialize, serde::Serialize, Default)]
pub struct Config {
    profile: HashMap<FileSafeString, ProfileSettings>,
}

impl Config {
    pub fn load() -> anyhow::Result<Self> {
        let config_path = config_file()?;

        // Treat missing config file as empty config
        if !config_path.exists() {
            return Ok(Config::default());
        }

        let config = std::fs::read_to_string(config_path)?;
        let config: Config = toml::from_str(&config)?;
        Ok(config)
    }

    fn save(&self) -> anyhow::Result<()> {
        let config_path = config_file()?;

        // ensure the directory exists
        if let Some(parent) = config_path.parent() {
            std::fs::create_dir_all(parent)?;
        }

        let config = toml::to_string_pretty(self)?;
        std::fs::write(config_path, config)?;
        Ok(())
    }

    fn get_profile(&self, name: FileSafeString) -> Profile {
        let settings = self.profile.get(&name).cloned();
        Profile { name, settings }
    }

    pub fn get_default_profile(&self) -> Profile {
        self.get_profile(DEFAULT_PROFILE.parse().unwrap())
    }
}

fn config_file() -> anyhow::Result<std::path::PathBuf> {
    let base = xdg::BaseDirectories::with_prefix(APP_NAME)?;
    Ok(base.get_cache_home().join("config.toml"))
}

fn cache_dir() -> anyhow::Result<std::path::PathBuf> {
    let base = xdg::BaseDirectories::with_prefix(APP_NAME)?;
    Ok(base.get_cache_home())
}

/// Prompt the user for their session token and save it to the config file.
///
/// cargo run -- login
/// Enter session cookie from https://adventofcode.com/ : <cookie>
/// Session cookie has been saved to <path>/.config/aoc/config.toml
pub fn login() -> Result<(), anyhow::Error> {
    println!("{}", include_str!("how_to_find_session_token.txt"));
    let session_token = rpassword::prompt_password("Enter session cookie: ")?;
    let mut config = Config::load()?;
    let default_profile = DEFAULT_PROFILE.parse().unwrap();
    let profile = config
        .profile
        .entry(default_profile)
        .or_insert_with(|| ProfileSettings {
            session_token: "".to_string(),
        });
    profile.session_token = session_token;
    config.save()?;
    println!(
        "Session cookie has been saved to {}",
        config_file()?.display()
    );
    Ok(())
}

fn get_or_create_cache(
    profile: &FileSafeString,
    year: usize,
    day: usize,
) -> anyhow::Result<std::path::PathBuf> {
    let profile_cache_dir = cache_dir()?.join("inputs").join(&profile.0);
    std::fs::create_dir_all(&profile_cache_dir)?;
    Ok(profile_cache_dir.join(format!("{}-{}.txt", year, day)))
}

pub struct Profile {
    name: FileSafeString,
    settings: Option<ProfileSettings>,
}

impl Profile {
    pub fn set_cached(&self, year: usize, day: usize, input: &str) -> anyhow::Result<()> {
        let location = get_or_create_cache(&self.name, year, day)?;
        std::fs::write(location, input)?;
        Ok(())
    }

    pub fn get_cached(&self, year: usize, day: usize) -> anyhow::Result<Option<String>> {
        let location = get_or_create_cache(&self.name, year, day)?;
        if !location.exists() {
            return Ok(None);
        }
        let input = std::fs::read_to_string(location)?;
        Ok(Some(input))
    }

    pub fn get_session_token(&self) -> Option<&str> {
        let settings = self.settings.as_ref()?;
        Some(&settings.session_token)
    }
}
