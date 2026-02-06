use std::ops::Deref;

use regex::Regex;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SerdeRegex(#[serde(with = "serde_regex")] Regex);

impl AsRef<Regex> for SerdeRegex {
  fn as_ref(&self) -> &Regex {
    &self.0
  }
}

impl Deref for SerdeRegex {
  type Target = Regex;

  fn deref(&self) -> &Self::Target {
    &self.0
  }
}

impl From<SerdeRegex> for Regex {
  fn from(value: SerdeRegex) -> Self {
    value.0
  }
}

impl From<Regex> for SerdeRegex {
  fn from(value: Regex) -> Self {
    SerdeRegex(value)
  }
}
