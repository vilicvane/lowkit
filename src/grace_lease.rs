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
    let timeout = self.timeout;
    let permit = self.semaphore.clone().acquire_owned().await.unwrap();

    GraceLeasePermit::new(permit, timeout)
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
