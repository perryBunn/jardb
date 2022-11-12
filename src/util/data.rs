use serde_derive::{Deserialize, Serialize};

// User data, which is stored and accessible in all command invocations
pub struct Data {}

#[derive(Serialize, Deserialize, Debug)]
pub struct Config {
    pub logging_level: String,
    pub(crate) token_file: String,
    pub bot: BotConfig
}

#[derive(Serialize, Deserialize, Debug)]
pub struct BotConfig {
    pub prefix: String,
    pub listen_timeout: u64,
}
