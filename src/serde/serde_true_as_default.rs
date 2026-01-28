use std::ops::Deref;

use serde::{Deserialize, Deserializer, Serialize};

use crate::SelfWrapExt;

#[derive(Debug, Clone, Serialize)]
#[serde(transparent)]
pub struct SerdeTrueAsDefault<T>(T);

impl<'de, T> Deserialize<'de> for SerdeTrueAsDefault<T>
where
  T: Deserialize<'de> + Default,
{
  fn deserialize<TDeserializer>(deserializer: TDeserializer) -> Result<Self, TDeserializer::Error>
  where
    TDeserializer: Deserializer<'de>,
  {
    SerdeTrueAsDefault(match TrueOrValue::<T>::deserialize(deserializer)? {
      TrueOrValue::True(true) => T::default(),
      TrueOrValue::True(false) => Err(serde::de::Error::custom("false is not allowed"))?,
      TrueOrValue::Value(value) => value,
    })
    .wrap_ok()
  }
}

#[derive(Deserialize)]
#[serde(untagged)]
enum TrueOrValue<T> {
  True(bool),
  Value(T),
}

impl<T> AsRef<T> for SerdeTrueAsDefault<T> {
  fn as_ref(&self) -> &T {
    &self.0
  }
}

impl<T> Deref for SerdeTrueAsDefault<T> {
  type Target = T;

  fn deref(&self) -> &Self::Target {
    &self.0
  }
}
