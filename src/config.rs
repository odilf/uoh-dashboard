use std::time::Duration;

use serde::Deserialize;

#[non_exhaustive]
#[derive(Debug, Clone, Copy, Deserialize)]
pub struct Config {
    pub base_refresh_period: Miliseconds,
}

#[derive(Debug, Clone, Copy, Deserialize)]
pub struct Miliseconds(u64);

impl From<Miliseconds> for Duration {
    fn from(value: Miliseconds) -> Self {
        Self::from_millis(value.0)
    }
}

impl Config {
    pub fn get() -> Self {
        let settings = config::Config::builder()
            .set_default("title_refresh_period", 5000)
            .unwrap()
            .set_default("base_refresh_period", 150)
            .unwrap()
            .add_source(config::Environment::with_prefix("UOH_DASHBOARD"))
            .build()
            .unwrap();

        settings.try_deserialize::<Config>().unwrap()
    }
}

impl Default for Config {
    fn default() -> Self {
        Self::get()
    }
}

#[test]
fn all_fields_have_defaults() {
    Config::get();
}
