pub struct DropCallback<T>
where
  T: FnOnce(),
{
  callback: Option<T>,
}

impl<T> DropCallback<T>
where
  T: FnOnce(),
{
  pub fn new(callback: T) -> Self {
    Self {
      callback: Some(callback),
    }
  }
}

impl<T> Drop for DropCallback<T>
where
  T: FnOnce(),
{
  fn drop(&mut self) {
    (self.callback.take().unwrap())();
  }
}
