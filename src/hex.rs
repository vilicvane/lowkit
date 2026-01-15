pub trait HexDecodeExt {
  fn hex_decode(&self) -> Result<Vec<u8>, hex::FromHexError>;
}

impl<T> HexDecodeExt for T
where
  T: AsRef<str>,
{
  fn hex_decode(&self) -> Result<Vec<u8>, hex::FromHexError> {
    hex::decode(self.as_ref())
  }
}

pub trait HexEncodeExt {
  fn hex_encode(&self) -> String;
}

impl<T> HexEncodeExt for T
where
  T: AsRef<[u8]>,
{
  fn hex_encode(&self) -> String {
    hex::encode(self.as_ref())
  }
}
