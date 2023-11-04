use super::api;
use super::persist::{Config, Profile};

pub struct Getter {
    profile: Profile,
}

impl Getter {
    pub fn load() -> anyhow::Result<Self> {
        let config = Config::load()?;
        let profile = config.get_default_profile();
        Ok(Self { profile })
    }

    pub fn get_input(&self, year: usize, day: usize) -> Result<String, api::Error> {
        // try getting from cache
        if let Some(input) = self.profile.get_cached(year, day)? {
            return Ok(input);
        }

        // otherwise, get from api
        let session_token = self.profile.get_session_token().ok_or(anyhow::anyhow!(
            "No session token found. Please run the login command."
        ))?;
        let ret = api::get_input(session_token, year, day)?;

        // cache it
        self.profile.set_cached(year, day, &ret)?;

        Ok(ret)
    }
}
