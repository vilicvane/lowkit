#[macro_export]
macro_rules! tokio_join_set {
  ($($task:expr),*) => {
    {
      let mut join_set = ::tokio::task::JoinSet::new();
      $(join_set.spawn($task);)*
      join_set
    }
  };
}
