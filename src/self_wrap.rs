use std::{
  cell::RefCell,
  future::{Ready, ready},
  rc::Rc,
  sync::{Arc, Mutex, RwLock},
};

pub trait SelfWrapExt: Sized {
  #[inline]
  fn rc(self) -> Rc<Self> {
    Rc::new(self)
  }

  #[inline]
  fn ref_cell(self) -> RefCell<Self> {
    RefCell::new(self)
  }

  #[inline]
  fn arc(self) -> Arc<Self> {
    Arc::new(self)
  }

  #[inline]
  fn mutex(self) -> Mutex<Self> {
    Mutex::new(self)
  }

  #[inline]
  fn tokio_mutex(self) -> tokio::sync::Mutex<Self> {
    tokio::sync::Mutex::new(self)
  }

  #[inline]
  fn rw_lock(self) -> RwLock<Self> {
    RwLock::new(self)
  }

  #[inline]
  fn tokio_rw_lock(self) -> tokio::sync::RwLock<Self> {
    tokio::sync::RwLock::new(self)
  }

  #[inline]
  fn some(self) -> Option<Self> {
    Some(self)
  }

  #[inline]
  fn ready(self) -> Ready<Self> {
    ready(self)
  }

  #[inline]
  fn wrap_box(self) -> Box<Self> {
    Box::new(self)
  }

  #[inline]
  fn wrap_ok<TError>(self) -> Result<Self, TError> {
    Ok(self)
  }

  #[inline]
  fn wrap_err<T>(self) -> Result<T, Self> {
    Err(self)
  }

  #[inline]
  fn anyhow<T, TError>(self) -> anyhow::Result<T>
  where
    Self: Into<Result<T, TError>>,
    TError: Into<anyhow::Error>,
  {
    self.into().map_err(|error| error.into())
  }

  #[inline]
  fn anyhow_ok(self) -> anyhow::Result<Self> {
    anyhow::Ok(self)
  }
}

impl<T> SelfWrapExt for T {}
