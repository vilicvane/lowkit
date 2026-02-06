use std::time::Duration;

use serde::{Deserialize, Deserializer, Serialize, Serializer};

use crate::SelfWrapExt;

#[derive(
  Debug, Clone, Copy, derive_more::AsRef, derive_more::Deref, derive_more::From, derive_more::Into,
)]
pub struct SerdeDuration(Duration);

impl Serialize for SerdeDuration {
  fn serialize<TSerializer>(
    &self,
    serializer: TSerializer,
  ) -> Result<TSerializer::Ok, TSerializer::Error>
  where
    TSerializer: Serializer,
  {
    serializer.serialize_str(&humantime::format_duration(self.0).to_string())
  }
}

impl<'de> Deserialize<'de> for SerdeDuration {
  fn deserialize<TDeserializer>(deserializer: TDeserializer) -> Result<Self, TDeserializer::Error>
  where
    TDeserializer: Deserializer<'de>,
  {
    SerdeDuration(
      humantime::parse_duration(&String::deserialize(deserializer)?)
        .map_err(serde::de::Error::custom)?,
    )
    .wrap_ok()
  }
}
