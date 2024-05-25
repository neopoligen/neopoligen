use anyhow::Result;
use rusqlite::Connection;

use crate::site_config::SiteConfig;

pub struct Builder {
    site_config: SiteConfig,
    conn: Option<Connection>,
}

impl Builder {
    pub fn new(site_config: SiteConfig) -> Result<Builder> {
        let conn = Connection::open(site_config.cache_db_path())?;
        Ok(Builder {
            site_config,
            conn: Some(conn),
        })
    }

    pub fn create_cache_db_if_necessary(self) -> Result<()> {
        let create_page_cache_table = "
        CREATE TABLE IF NOT EXISTS pages (source_path TEXT, source_content TEXT, source_ast TEXT)";
        self.conn.unwrap().execute(create_page_cache_table, ())?;
        Ok(())
    }
}
