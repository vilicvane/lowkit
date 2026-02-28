use std::net::SocketAddr;

use serde::{Deserialize, Deserializer, Serialize, Serializer};

use crate::SelfWrapExt;

#[derive(
  Debug, Clone, Copy, derive_more::AsRef, derive_more::Deref, derive_more::From, derive_more::Into,
)]
pub struct SerdeSocketAddress(SocketAddr);

impl SerdeSocketAddress {
  pub fn into_inner(self) -> SocketAddr {
    self.0
  }
}

impl Serialize for SerdeSocketAddress {
  fn serialize<TSerializer>(
    &self,
    serializer: TSerializer,
  ) -> Result<TSerializer::Ok, TSerializer::Error>
  where
    TSerializer: Serializer,
  {
    serializer.serialize_str(&self.0.to_string())
  }
}

impl<'de> Deserialize<'de> for SerdeSocketAddress {
  fn deserialize<TDeserializer>(deserializer: TDeserializer) -> Result<Self, TDeserializer::Error>
  where
    TDeserializer: Deserializer<'de>,
  {
    SerdeSocketAddress(
      String::deserialize(deserializer)?
        .parse()
        .map_err(serde::de::Error::custom)?,
    )
    .wrap_ok()
  }
}
