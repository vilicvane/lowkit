use std::{
  sync::{
    Arc,
    atomic::{self, AtomicUsize},
  },
  time::Duration,
};

use tokio::{sync::Notify, task::JoinSet};

use crate::{SelfWrapExt, tokio_join_set};

struct DebouncedCountLoggerInner {
  message: String,
  count: AtomicUsize,
  debounce_duration: Duration,
  notify: Notify,
}

impl DebouncedCountLoggerInner {
  fn log(&self) {
    let count = self.count.swap(0, atomic::Ordering::Relaxed);

    if count > 0 {
      log::info!("{}", self.message.replace("{count}", &count.to_string()));
    }
  }
}

pub struct DebouncedCountLogger {
  inner: Arc<DebouncedCountLoggerInner>,
  _join_set: JoinSet<()>,
}

impl DebouncedCountLogger {
  pub fn new(message: impl Into<String>, debounce_duration: Duration) -> Self {
    let inner = DebouncedCountLoggerInner {
      message: message.into(),
      count: AtomicUsize::new(0),
      debounce_duration,
      notify: Notify::new(),
    }
    .arc();

    Self {
      inner: inner.clone(),
      _join_set: tokio_join_set!(async move {
        loop {
          inner.notify.notified().await;

          while tokio::time::timeout(inner.debounce_duration, inner.notify.notified())
            .await
            .is_ok()
          {}

          inner.log();
        }
      }),
    }
  }

  pub fn count(&self) {
    self.inner.count.fetch_add(1, atomic::Ordering::Relaxed);
    self.inner.notify.notify_one();
  }

  pub fn add(&self, count: usize) {
    self.inner.count.fetch_add(count, atomic::Ordering::Relaxed);
    self.inner.notify.notify_one();
  }

  pub fn update(&self, count: usize) {
    self.inner.count.store(count, atomic::Ordering::Relaxed);
    self.inner.notify.notify_one();
  }

  pub fn reset(&self) {
    self.inner.count.store(0, atomic::Ordering::Relaxed);
  }
}

impl Drop for DebouncedCountLogger {
  fn drop(&mut self) {
    self.inner.log();
  }
}
