use std::{sync::Arc, time::Duration};

use tokio::{
  sync::{OwnedSemaphorePermit, Semaphore},
  time::sleep,
};

pub struct GraceLease {
  semaphore: Arc<Semaphore>,
  timeout: Duration,
}

impl GraceLease {
  pub fn new(permits: usize, timeout: Duration) -> Self {
    Self {
      semaphore: Arc::new(Semaphore::new(permits)),
      timeout,
    }
  }

  pub async fn acquire(&self) -> GraceLeasePermit {
    self.acquire_with_grace_period(self.timeout).await
  }

  /// Acquires from the same pool, using a per-permit cooldown after it is dropped.
  pub async fn acquire_with_grace_period(&self, grace_period: Duration) -> GraceLeasePermit {
    let permit = self.semaphore.clone().acquire_owned().await.unwrap();

    GraceLeasePermit::new(permit, grace_period)
  }
}

pub struct GraceLeasePermit {
  permit: Option<OwnedSemaphorePermit>,
  timeout: Duration,
}

impl GraceLeasePermit {
  pub fn new(permit: OwnedSemaphorePermit, timeout: Duration) -> Self {
    Self {
      permit: Some(permit),
      timeout,
    }
  }
}

impl Drop for GraceLeasePermit {
  fn drop(&mut self) {
    let permit = self.permit.take().unwrap();
    let timeout = self.timeout;

    tokio::spawn(async move {
      sleep(timeout).await;
      drop(permit);
    });
  }
}
