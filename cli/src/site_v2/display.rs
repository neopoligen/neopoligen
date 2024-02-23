use crate::site_v2::SiteV2;
use std::fmt::Display;

// This function is required by minijinja. The value shows
// up if you call {{ site_v2 }}. I'm setting it to nothing
// since I don't use that.

impl Display for SiteV2 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "")
    }
}
