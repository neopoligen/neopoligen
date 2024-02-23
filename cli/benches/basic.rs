use dirs::config_dir;
use dirs::document_dir;
use neopoligen_cli::config::Config;
use neopoligen_cli::site_builder::SiteBuilder;
use serde::Deserialize;
use serde_json;
use std::fs;

fn main() {
    // Run registered benchmarks.
    divan::main();
}

#[derive(Deserialize, Debug)]
pub struct NeoConfig {
    active_site: String,
}

// Register a `fibonacci` function and benchmark it over multiple cases.
#[divan::bench()]
fn do_test() {
    let mut neo_config_file = config_dir().unwrap();
    neo_config_file.push("Neopoligen/config.json");

    match fs::read_to_string(&neo_config_file) {
        Ok(neo_config_string) => match serde_json::from_str::<NeoConfig>(&neo_config_string) {
            Ok(neo_config) => {
                let mut site_root = document_dir().unwrap();
                site_root.push("Neopoligen");
                site_root.push("sites");
                site_root.push(neo_config.active_site);
                let site_config = Config::new(site_root);
                let mut site_builder = SiteBuilder::new(site_config);
                site_builder.build_site();
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

// fn fibonacci(n: u64) -> u64 {
//     if n <= 1 {
//         1
//     } else {
//         fibonacci(n - 2) + fibonacci(n - 1)
//     }
// }
