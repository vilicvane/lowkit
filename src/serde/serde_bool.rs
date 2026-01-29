use serde::{Deserialize, Deserializer, Serialize, de::Error};

use crate::SelfWrapExt;

#[derive(Debug, Clone, Copy, Default, Serialize)]
pub struct SerdeTrue;

impl<'de> Deserialize<'de> for SerdeTrue {
  fn deserialize<TDeserializer>(deserializer: TDeserializer) -> Result<Self, TDeserializer::Error>
  where
    TDeserializer: Deserializer<'de>,
  {
    match bool::deserialize(deserializer)? {
      true => SerdeTrue.wrap_ok(),
      false => Err(TDeserializer::Error::custom("expected true, got false")),
    }
  }
}

#[derive(Debug, Clone, Copy, Default, Serialize)]
pub struct SerdeFalse;

impl<'de> Deserialize<'de> for SerdeFalse {
  fn deserialize<TDeserializer>(deserializer: TDeserializer) -> Result<Self, TDeserializer::Error>
  where
    TDeserializer: Deserializer<'de>,
  {
    match bool::deserialize(deserializer)? {
      false => SerdeFalse.wrap_ok(),
      true => Err(TDeserializer::Error::custom("expected false, got true")),
    }
  }
}

impl From<SerdeTrue> for bool {
  fn from(_: SerdeTrue) -> Self {
    true
  }
}

impl From<SerdeFalse> for bool {
  fn from(_: SerdeFalse) -> Self {
    false
  }
}
