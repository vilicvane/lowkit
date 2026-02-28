use serde::{Deserialize, Deserializer, Serialize, Serializer};

use crate::SelfWrapExt;

#[derive(
  Debug, Clone, derive_more::AsRef, derive_more::Deref, derive_more::From, derive_more::Into,
)]
pub struct SerdeBraceExpandedStrings(Vec<String>);

impl SerdeBraceExpandedStrings {
  pub fn into_inner(self) -> Vec<String> {
    self.0
  }
}

impl Serialize for SerdeBraceExpandedStrings {
  fn serialize<TSerializer>(
    &self,
    serializer: TSerializer,
  ) -> Result<TSerializer::Ok, TSerializer::Error>
  where
    TSerializer: Serializer,
  {
    self.0.serialize(serializer)
  }
}

impl<'de> Deserialize<'de> for SerdeBraceExpandedStrings {
  fn deserialize<TDeserializer>(deserializer: TDeserializer) -> Result<Self, TDeserializer::Error>
  where
    TDeserializer: Deserializer<'de>,
  {
    let pattern = String::deserialize(deserializer)?;

    let expression: bexpand::Expression = pattern.parse().map_err(serde::de::Error::custom)?;

    SerdeBraceExpandedStrings(
      expression
        .into_iter()
        .map(|result| result.map(|cow| cow.into_owned()))
        .collect::<Result<Vec<String>, _>>()
        .map_err(serde::de::Error::custom)?,
    )
    .wrap_ok()
  }
}
