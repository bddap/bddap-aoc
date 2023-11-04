use chrono::{DateTime, TimeZone, Utc};

pub enum Error {
    ChallengeNotReady {
        time_till_ready: std::time::Duration,
    },
    Other(anyhow::Error),
}

impl<E> From<E> for Error
where
    E: Into<anyhow::Error>,
{
    fn from(e: E) -> Self {
        Error::Other(e.into())
    }
}

/// challenges are released at midnight EST (UTC-5)
/// get the time this challenge will be released
fn release_time(year: usize, day: usize) -> DateTime<Utc> {
    let est = chrono::FixedOffset::west_opt(5 * 60 * 60).unwrap();

    let hour = 0;
    let minute = 0;
    let second = 0;
    est.with_ymd_and_hms(year as i32, 12, day as u32, hour, minute, second)
        .unwrap()
        .into()
}

pub fn to_approx_std_duration(d: chrono::Duration) -> std::time::Duration {
    std::time::Duration::from_secs(d.num_seconds().try_into().unwrap_or(0))
}

pub fn get_input(session_token: &str, year: usize, day: usize) -> Result<String, Error> {
    let release_time = release_time(year, day);
    let now = Utc::now();
    if now < release_time {
        let time_till_ready = to_approx_std_duration(release_time - now);
        return Err(Error::ChallengeNotReady { time_till_ready });
    }

    let input = reqwest::blocking::Client::new()
        .get(format!(
            "https://adventofcode.com/{}/day/{}/input",
            year, day
        ))
        .header("Cookie", format!("session={}", session_token))
        .send()?
        .error_for_status()?
        .text()?;
    Ok(input)
}
