#[derive(thiserror::Error, Debug)]
#[error("user interruption")]
pub struct UserInterruption;

pub fn user_interrupt() -> Result<(), UserInterruption> {
  Err(UserInterruption)
}
