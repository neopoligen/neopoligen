use crate::site::Site;
use std::fmt::Display;

impl Display for Site {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        // NOTE: this function is required by minijinja. It's what
        // shows up when calling `{{ site }}`` which I just set to empty
        write!(f, "")
    }
}
