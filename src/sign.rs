#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, derive_more::Display)]
pub enum Sign {
  #[display("-")]
  Negative,
  #[display("")]
  Zero,
  #[display("+")]
  Positive,
}

impl std::ops::Neg for Sign {
  type Output = Sign;

  fn neg(self) -> Self::Output {
    match self {
      Sign::Positive => Sign::Negative,
      Sign::Negative => Sign::Positive,
      Sign::Zero => Sign::Zero,
    }
  }
}

macro_rules! impl_sign {
  ($type:ty, $zero:expr, $one:expr) => {
    impl From<$type> for Sign {
      fn from(value: $type) -> Self {
        if value > $zero {
          Sign::Positive
        } else if value < $zero {
          Sign::Negative
        } else {
          Sign::Zero
        }
      }
    }

    impl std::ops::Mul<$type> for Sign {
      type Output = $type;

      fn mul(self, rhs: $type) -> Self::Output {
        rhs
          * match self {
            Sign::Positive => $one,
            Sign::Negative => -$one,
            Sign::Zero => $zero,
          }
      }
    }
  };
}

impl_sign!(i8, 0, 1);
impl_sign!(i16, 0, 1);
impl_sign!(i32, 0, 1);
impl_sign!(i64, 0, 1);
impl_sign!(i128, 0, 1);
impl_sign!(isize, 0, 1);
impl_sign!(f32, 0.0, 1.0);
impl_sign!(f64, 0.0, 1.0);
