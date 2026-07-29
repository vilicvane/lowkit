use std::{
  sync::{
    Arc,
    atomic::{self, AtomicUsize},
  },
  time::Duration,
};

use tokio::task::JoinSet;

use crate::{SelfWrapExt, tokio_join_set};

struct PeriodicCountLoggerInner {
  message: String,
  count: AtomicUsize,
}

impl PeriodicCountLoggerInner {
  fn log(&self) {
    let count = self.count.swap(0, atomic::Ordering::Relaxed);

    log::info!("{}", self.message.replace("{count}", &count.to_string()));
  }
}

/// 与 [`DebouncedCountLogger`](crate::DebouncedCountLogger) 相对：按固定周期输出
/// 计数，适合事件本身按周期发生的场景（如每秒快照）。每个周期都输出一行，
/// 包括计数为 0 时——周期内没有事件恰恰是该被看见的异常。
pub struct PeriodicCountLogger {
  inner: Arc<PeriodicCountLoggerInner>,
  _join_set: JoinSet<()>,
}

impl PeriodicCountLogger {
  pub fn new(message: impl Into<String>, interval: Duration) -> Self {
    let inner = PeriodicCountLoggerInner {
      message: message.into(),
      count: AtomicUsize::new(0),
    }
    .arc();

    Self {
      inner: inner.clone(),
      _join_set: tokio_join_set!(async move {
        let mut interval = tokio::time::interval(interval);

        // tokio interval 的首个 tick 立即返回，吞掉它，避免启动时输出一行
        // 无意义的 0 计数。
        interval.tick().await;

        loop {
          interval.tick().await;
          inner.log();
        }
      }),
    }
  }

  pub fn count(&self) {
    self.inner.count.fetch_add(1, atomic::Ordering::Relaxed);
  }

  pub fn add(&self, count: usize) {
    self.inner.count.fetch_add(count, atomic::Ordering::Relaxed);
  }

  pub fn update(&self, count: usize) {
    self.inner.count.store(count, atomic::Ordering::Relaxed);
  }

  pub fn reset(&self) {
    self.inner.count.store(0, atomic::Ordering::Relaxed);
  }
}

impl Drop for PeriodicCountLogger {
  fn drop(&mut self) {
    self.inner.log();
  }
}
