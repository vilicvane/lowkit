use std::str::FromStr;

use crate::SelfWrapExt;

#[derive(
  Debug, Clone, derive_more::AsRef, derive_more::Deref, derive_more::From, derive_more::Into,
)]
pub struct BraceExpandedStringsFromStr(Vec<String>);

impl BraceExpandedStringsFromStr {
  pub fn into_inner(self) -> Vec<String> {
    self.0
  }
}

impl FromStr for BraceExpandedStringsFromStr {
  type Err = String;

  fn from_str(pattern: &str) -> Result<Self, Self::Err> {
    let expression: bexpand::Expression = pattern.parse()?;

    BraceExpandedStringsFromStr(
      expression
        .into_iter()
        .map(|result| result.map(|cow| cow.into_owned()))
        .collect::<Result<Vec<String>, _>>()
        .map_err(|error| error.to_string())?,
    )
    .wrap_ok()
  }
}
