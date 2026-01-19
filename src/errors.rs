use crate::SelfWrapExt;

#[derive(thiserror::Error, Debug)]
#[error("user interruption")]
pub struct UserInterruption;

pub fn user_interrupt() -> Result<(), UserInterruption> {
  Err(UserInterruption)
}

pub trait UserInterruptExt<TOk>
where
  Self: Into<Result<TOk, anyhow::Error>>,
  TOk: Default,
{
  fn user_interrupt_ok(self) -> anyhow::Result<TOk> {
    self.into().or_else(|error| {
      if error.is::<UserInterruption>() {
        TOk::default().wrap_ok()
      } else {
        error.wrap_err()
      }
    })
  }
}

impl<T, TOk> UserInterruptExt<TOk> for T
where
  T: Into<Result<TOk, anyhow::Error>>,
  TOk: Default,
{
}
