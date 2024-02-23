use crate::site::Site;
use std::fmt::Display;

impl Display for Site {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        // NOTE: This function is required by minijinjs. I tired
        // putting the page ID in it, but it didn't work as expected
        // when trying to use it for page comparisons. I'm using
        // page.id() explicitly instead.
        write!(f, "")
    }
}

