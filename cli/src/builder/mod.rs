use crate::site_config::SiteConfig;
use tracing::{event, instrument, Level};

pub struct Builder {
    config: Option<SiteConfig>,
}

impl Builder {
    #[instrument(skip(thing))]
    pub fn todo(thing: &str) {
        event!(Level::INFO, "TODO: {}", thing);
    }
}

