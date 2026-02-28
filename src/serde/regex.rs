use regex::Regex;
use serde::{Deserialize, Serialize};

#[derive(
  Debug,
  Clone,
  Serialize,
  Deserialize,
  derive_more::AsRef,
  derive_more::Deref,
  derive_more::From,
  derive_more::Into,
)]
pub struct SerdeRegex(#[serde(with = "serde_regex")] Regex);

impl SerdeRegex {
  pub fn into_inner(self) -> Regex {
    self.0
  }
}
