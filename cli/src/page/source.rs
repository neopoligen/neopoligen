use crate::page::Page;

impl Page {
    pub fn source(&self) -> String {
        self.source.clone()
    }
}

#[cfg(test)]
mod test {
  // this just straight returns the source
  // no testing for that
}
