use std::sync::{Arc, Weak};

pub enum TurnArcWeak<T> {
  Arc(Arc<T>),
  Weak(Weak<T>),
}

impl<T> TurnArcWeak<T> {
  pub fn new(value: T) -> Self {
    Self::Arc(Arc::new(value))
  }

  pub fn get_arc(&mut self) -> Option<Arc<T>> {
    let arc = match self {
      Self::Arc(arc) => arc.clone(),
      Self::Weak(weak) => return weak.upgrade(),
    };

    *self = Self::Weak(Arc::downgrade(&arc));

    Some(arc)
  }
}
