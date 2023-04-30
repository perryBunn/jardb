use log::LevelFilter;
use serde_derive::{Deserialize, Serialize};

// User data, which is stored and accessible in all command invocations
pub struct Data {}

#[derive(Serialize, Deserialize, Debug)]
pub struct Config {
    pub bot: BotConfig,
    pub logging_level: LevelFilter,
    pub(crate) token_file: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct BotConfig {
    pub listen_timeout: u64,
    pub name: String,
    pub prefix: String,
}
