use serde::Deserialize;

#[derive(Deserialize, Clone, Debug)]
pub struct NeoConfig {
    pub dev: NeoEnv,
    pub prod: NeoEnv,
}

#[derive(Deserialize, Clone, Debug)]
pub struct NeoEnv {
    pub active_site: Option<String>,
}
