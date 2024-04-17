use serde::Deserialize;

#[derive(Deserialize, Clone)]
pub struct NeoConfig {
    pub dev: NeoEnv,
    pub prod: NeoEnv,
}

#[derive(Deserialize, Clone)]
pub struct NeoEnv {
    pub active_site: Option<String>,
}
