// DEPRECATED: TODO: Moving to using NeoError
use minijinja::value::{Object, Value};
use minijinja::Error;
use serde::Serialize;
use std::fmt::Display;
use std::path::PathBuf;
use std::sync::Arc;

#[derive(Clone, Debug, Serialize)]
pub struct BuildIssue {
    pub details: Option<String>,
    pub source_path: Option<PathBuf>,
    pub kind: BuildIssueKind,
}

#[derive(Clone, Debug, Serialize)]
#[serde(tag = "kind", rename_all = "lowercase")]
pub enum BuildIssueKind {
    CouldNotReadThemeTest {},
    CouldNotRenderThemeTest {},
    FailedThemeTest {
        expected: Option<String>,
        got: Option<String>,
    },
    Generic {},
    InvalidThemeTest {},
    MissingPageId {},
    NoThemeTestsFound {},
}

impl BuildIssue {
    pub fn expected(&self) -> Result<Value, Error> {
        match &self.kind {
            BuildIssueKind::FailedThemeTest { expected, .. } => {
                Ok(Value::from(expected.clone().unwrap()))
            }
            _ => Err(Error::new(
                minijinja::ErrorKind::CannotUnpack,
                "nothing to get",
            )),
        }
    }

    pub fn file_name(&self) -> Result<Value, Error> {
        Ok(Value::from("FILENAME"))
    }

    pub fn kind(&self) -> Result<Value, Error> {
        Ok(Value::from_serialize(&self.kind))
    }

    pub fn got(&self) -> Result<Value, Error> {
        match &self.kind {
            BuildIssueKind::FailedThemeTest { got, .. } => Ok(Value::from(got.clone().unwrap())),
            _ => Err(Error::new(
                minijinja::ErrorKind::CannotUnpack,
                "nothing to get",
            )),
        }
    }
}

impl Object for BuildIssue {
    fn call_method(
        self: &Arc<BuildIssue>,
        _state: &minijinja::State,
        name: &str,
        _args: &[Value],
    ) -> Result<Value, Error> {
        match name {
            "expected" => self.expected(),
            "file_name" => self.file_name(),
            "kind" => self.kind(),
            "got" => self.got(),
            _ => Ok(Value::from("[Error: called non-existing function")),
        }
    }
}

impl Display for BuildIssue {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "builder_issue")
    }
}
