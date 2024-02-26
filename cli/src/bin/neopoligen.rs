use dirs::document_dir;
use neopoligen::config::Config;
use serde::Deserialize;
use std::fs;

#[derive(Deserialize)]
pub struct EngineConfig {
    settings: EngineConfigSettings,
}

#[derive(Deserialize)]
pub struct EngineConfigSettings {
    active_site: String,
}

fn main() {
    let mut engine_config_file = document_dir().unwrap();
    engine_config_file.push("Neopoligen");
    engine_config_file.push("config.toml");
    match fs::read_to_string(&engine_config_file) {
        Ok(engine_config_string) => match toml::from_str::<EngineConfig>(&engine_config_string) {
            Ok(engine_config) => {
                let mut site_root = document_dir().unwrap();
                site_root.push("Neopoligen");
                site_root.push(engine_config.settings.active_site);
                let site_config = Config::new(site_root);

                // build_site(site_config.clone());
                // // this is the tmp flag to turn off the watch without getting a warning
                // if true {
                //     run_web_server(site_config).await;
                // }
            }
            Err(e) => {
                println!("{}", e)
            }
        },

        Err(e) => {
            println!("{}", e)
        }
    }
}
