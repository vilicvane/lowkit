use std::borrow::Borrow;

pub enum Boo<'a, T> {
  Borrowed(&'a T),
  Owned(T),
}

impl<T> Borrow<T> for Boo<'_, T> {
  fn borrow(&self) -> &T {
    match self {
      Self::Borrowed(reference) => reference,
      Self::Owned(value) => value,
    }
  }
}
