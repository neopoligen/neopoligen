use crate::ast::*;
use crate::child::*;
use crate::config::Config;
use tracing::instrument;

#[instrument]
pub fn parse(source: &str, config: &Config) -> Vec<Child> {
    if let Ok((_, ast)) = ast(source.trim_start(), config) {
        ast
    } else {
        vec![]
    }
}
