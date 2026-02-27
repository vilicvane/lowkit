use std::{cmp::Ordering, ops::Mul};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, derive_more::Display)]
pub enum Order {
  #[display("asc")]
  Ascending,
  #[display("desc")]
  Descending,
}

impl Mul<Order> for Ordering {
  type Output = Ordering;

  fn mul(self, rhs: Order) -> Self::Output {
    match rhs {
      Order::Ascending => self,
      Order::Descending => self.reverse(),
    }
  }
}
