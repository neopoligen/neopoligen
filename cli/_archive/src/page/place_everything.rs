use crate::child::Child;
use crate::page::Page;
use tracing::instrument;

impl Page {
    #[instrument]
    pub fn place_everything(&self) -> Vec<Child> {
        self.ast.clone()
    }
}
