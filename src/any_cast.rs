/// Tries multiple concrete types and returns a shared reference to a target trait object.
///
/// Candidate concrete types are checked in order with `Any::downcast_ref`.
/// The first successful match is coerced to `&$target_type`.
///
/// This is useful when a value is type-erased at the callsite, but you still know
/// a small set of concrete types that may implement the target trait.
///
/// # Example
///
/// ```rust
/// use lowkit::any_cast_ref;
///
/// trait Animal {
///   fn sound(&self) -> &'static str;
/// }
///
/// struct Cat;
/// impl Animal for Cat {
///   fn sound(&self) -> &'static str {
///     "meow"
///   }
/// }
///
/// struct Dog;
/// impl Animal for Dog {
///   fn sound(&self) -> &'static str {
///     "woof"
///   }
/// }
///
/// let cat = Cat;
/// let animal = any_cast_ref!(&cat => dyn Animal; Dog, Cat).unwrap();
///
/// assert_eq!(animal.sound(), "meow");
/// ```
///
/// # Notes
///
/// The input expression must evaluate to a reference to a concrete `'static` type
/// so it can be cast to `&dyn Any` internally.
#[macro_export]
macro_rules! any_cast_ref {
  ($value:expr => $target_type:ty; $($source_type:ty),+ $(,)?) => {{
    'any_cast: {
      let value = $value as &dyn ::core::any::Any;

      $(
        if let Some(value) = value.downcast_ref::<$source_type>() {
          break 'any_cast ::core::option::Option::Some(value as &$target_type);
        }
      )*

      ::core::option::Option::<&$target_type>::None
    }
  }};
}

/// Tries multiple concrete types and returns a mutable reference to a target trait object.
///
/// Candidate concrete types are checked in order with `Any::downcast_mut`.
/// The first successful match is coerced to `&mut $target_type`.
///
/// # Example
///
/// ```rust
/// use lowkit::any_cast_mut;
///
/// trait Animal {
///   fn rename(&mut self, name: &'static str);
///   fn name(&self) -> &'static str;
/// }
///
/// struct Dog {
///   name: &'static str,
/// }
///
/// impl Animal for Dog {
///   fn rename(&mut self, name: &'static str) {
///     self.name = name;
///   }
///
///   fn name(&self) -> &'static str {
///     self.name
///   }
/// }
///
/// let mut dog = Dog { name: "spot" };
/// let animal = any_cast_mut!(&mut dog => dyn Animal; Dog).unwrap();
/// animal.rename("buddy");
///
/// assert_eq!(dog.name, "buddy");
/// ```
///
/// # Notes
///
/// The input expression must evaluate to a mutable reference to a concrete `'static`
/// type so it can be cast to `&mut dyn Any` internally.
#[macro_export]
macro_rules! any_cast_mut {
  ($value:expr => $target_type:ty; $($source_type:ty),+ $(,)?) => {{
    'any_cast: {
      let value = $value as &mut dyn ::core::any::Any;

      $(
        if let Some(value) = value.downcast_mut::<$source_type>() {
          break 'any_cast ::core::option::Option::Some(value as &mut $target_type);
        }
      )*

      ::core::option::Option::<&mut $target_type>::None
    }
  }};
}

#[cfg(test)]
mod tests {
  trait TargetTrait {
    fn kind(&self) -> &'static str;
    fn set_kind(&mut self, kind: &'static str);
  }

  struct ConcreteTypeA {
    kind: &'static str,
  }

  impl TargetTrait for ConcreteTypeA {
    fn kind(&self) -> &'static str {
      self.kind
    }

    fn set_kind(&mut self, kind: &'static str) {
      self.kind = kind;
    }
  }

  struct ConcreteTypeB {
    kind: &'static str,
  }

  impl TargetTrait for ConcreteTypeB {
    fn kind(&self) -> &'static str {
      self.kind
    }

    fn set_kind(&mut self, kind: &'static str) {
      self.kind = kind;
    }
  }

  struct Wallet;

  #[test]
  fn any_cast_ref_matches_supported_type() {
    let destination = ConcreteTypeA { kind: "type-a" };

    let account = any_cast_ref!(&destination => dyn TargetTrait; ConcreteTypeB, ConcreteTypeA).unwrap();

    assert_eq!(account.kind(), "type-a");
  }

  #[test]
  fn any_cast_ref_returns_none_when_no_type_matches() {
    let destination = Wallet;

    let account = any_cast_ref!(&destination => dyn TargetTrait; ConcreteTypeA, ConcreteTypeB);

    assert!(account.is_none());
  }

  #[test]
  fn any_cast_mut_allows_mutating_through_target_trait() {
    let mut destination = ConcreteTypeB { kind: "before" };

    let account =
      any_cast_mut!(&mut destination => dyn TargetTrait; ConcreteTypeA, ConcreteTypeB).unwrap();

    account.set_kind("after");

    assert_eq!(destination.kind, "after");
  }
}
