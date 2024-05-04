use serde::Deserialize;
use serde::Serialize;
//use std::collections::BTreeMap;

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[serde(tag = "type", rename_all = "lowercase")]
pub struct EngineConfig {
    pub dev: EngineConfigEnv,
    pub prod: EngineConfigEnv,
}

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[serde(tag = "type", rename_all = "lowercase")]
pub struct EngineConfigEnv {
    pub active_site: String,
}
