// DEPRECATE this, i think it might already not be in play
use crate::ast::*;
use crate::child::*;
use crate::config::Config;
use tracing::instrument;

#[instrument]
pub fn parse(source: &str, config: &Config) -> Vec<Child> {
    match ast(source.trim_start(), config) {
        Ok((_remainder, ast)) => ast,
        Err(e) => {
            println!("{}", e);
            vec![]
        }
    }
    // if let Ok((_, ast)) = ast(source.trim_start(), config) {
    //     ast
    // } else {
    //     vec![]
    // }
}
