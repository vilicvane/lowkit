use std::{net::SocketAddr, ops::Deref};

use serde::{Deserialize, Deserializer, Serialize, Serializer};

use crate::SelfWrapExt;

#[derive(Debug, Clone)]
pub struct SerdeSocketAddress(SocketAddr);

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

impl AsRef<SocketAddr> for SerdeSocketAddress {
  fn as_ref(&self) -> &SocketAddr {
    &self.0
  }
}

impl Deref for SerdeSocketAddress {
  type Target = SocketAddr;

  fn deref(&self) -> &Self::Target {
    &self.0
  }
}

impl From<SerdeSocketAddress> for SocketAddr {
  fn from(value: SerdeSocketAddress) -> Self {
    value.0
  }
}

impl From<SocketAddr> for SerdeSocketAddress {
  fn from(value: SocketAddr) -> Self {
    SerdeSocketAddress(value)
  }
}
