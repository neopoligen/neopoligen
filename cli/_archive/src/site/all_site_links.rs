use crate::site::Site;

// TODO: Sort this chronologically
//
// TODO: Make this match the Value return
// format of the filtered links

impl Site {
    pub fn all_site_links(&self) -> Vec<(String, String)> {
        self.page_data
            .iter()
            .map(|pd| (pd.1.url_path.clone().unwrap(), "b".to_string()))
            .collect()
    }
}
