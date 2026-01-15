use base64::Engine;

pub trait Base64DecodeExt {
  fn base64_decode(&self) -> Result<Vec<u8>, base64::DecodeError>;
}

impl<T> Base64DecodeExt for T
where
  T: AsRef<str>,
{
  fn base64_decode(&self) -> Result<Vec<u8>, base64::DecodeError> {
    base64::engine::general_purpose::STANDARD.decode(self.as_ref())
  }
}

pub trait Base64EncodeExt {
  fn base64_encode(&self) -> String;
}

impl<T> Base64EncodeExt for T
where
  T: AsRef<[u8]>,
{
  fn base64_encode(&self) -> String {
    base64::engine::general_purpose::STANDARD.encode(self.as_ref())
  }
}
