/// Matches a shared type-erased reference against multiple concrete types via `Any`.
///
/// The input expression must evaluate to `&T` so it can be cast to `&dyn Any`
/// internally. Each arm declares the concrete type to try and a binding that
/// receives the matched shared reference.
///
/// When no fallback arm is provided, the macro returns `Option<T>` where `T` is
/// the common type produced by the arms.
///
/// # Examples
///
/// ```rust
/// use lowkit::any_cast_match;
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
///
/// let sound = any_cast_match!(&cat;
///   Dog => |dog| dog.sound(),
///   Cat => |cat| cat.sound(),
/// );
///
/// assert_eq!(sound, Some("meow"));
/// ```
#[macro_export]
macro_rules! any_cast_match {
  ($value:expr; $($rest:tt)*) => {{
    let value = $value as &dyn ::core::any::Any;
    $crate::any_cast_match!(@collect value; [] $($rest)*)
  }};

  (@collect $value:ident; [$($arms:tt)*] _ => $fallback:expr $(,)?) => {
    $crate::any_cast_match!(@emit_fallback $value; [$($arms)*] $fallback)
  };
  (@collect $value:ident; [$($arms:tt)*] $source_type:ty => |$binding:ident| $body:expr, $($rest:tt)+) => {
    $crate::any_cast_match!(@collect $value; [$($arms)* [$source_type][$binding][$body]] $($rest)+)
  };
  (@collect $value:ident; [$($arms:tt)*] $source_type:ty => |$binding:ident| $body:expr $(,)?) => {
    $crate::any_cast_match!(@emit_option $value; [$($arms)* [$source_type][$binding][$body]])
  };

  (@emit_fallback $value:ident; [] $fallback:expr) => {
    $fallback
  };
  (@emit_fallback $value:ident; [[$source_type:ty][$binding:ident][$body:expr] $($rest:tt)*] $fallback:expr) => {{
    if let Some($binding) = $value.downcast_ref::<$source_type>() {
      $body
    } else {
      $crate::any_cast_match!(@emit_fallback $value; [$($rest)*] $fallback)
    }
  }};

  (@emit_option $value:ident; [[$source_type:ty][$binding:ident][$body:expr] $($rest:tt)*]) => {{
    if let Some($binding) = $value.downcast_ref::<$source_type>() {
      ::core::option::Option::Some($body)
    } else {
      $crate::any_cast_match!(@emit_option_rest $value; [$($rest)*])
    }
  }};
  (@emit_option_rest $value:ident; []) => {
    ::core::option::Option::None
  };
  (@emit_option_rest $value:ident; [[$source_type:ty][$binding:ident][$body:expr] $($rest:tt)*]) => {{
    if let Some($binding) = $value.downcast_ref::<$source_type>() {
      ::core::option::Option::Some($body)
    } else {
      $crate::any_cast_match!(@emit_option_rest $value; [$($rest)*])
    }
  }};
}

/// Matches a mutable type-erased reference against multiple concrete types via
/// `Any`.
///
/// The input expression must evaluate to `&mut T` so it can be cast to
/// `&mut dyn Any` internally. Each arm declares the concrete type to try and a
/// binding that receives the matched mutable reference.
///
/// When no fallback arm is provided, the macro returns `Option<T>` where `T` is
/// the common type produced by the arms.
///
/// # Examples
///
/// ```rust
/// use lowkit::any_cast_match_mut;
///
/// struct Cat {
///   name: &'static str,
/// }
///
/// struct Dog {
///   name: &'static str,
/// }
///
/// let mut dog = Dog { name: "spot" };
///
/// any_cast_match_mut!(&mut dog;
///   Cat => |cat| cat.name = "kitty",
///   Dog => |dog| dog.name = "buddy",
/// )
/// .unwrap();
///
/// assert_eq!(dog.name, "buddy");
/// ```
#[macro_export]
macro_rules! any_cast_match_mut {
  ($value:expr; $($rest:tt)*) => {{
    let value = $value as &mut dyn ::core::any::Any;
    $crate::any_cast_match_mut!(@collect value; [] $($rest)*)
  }};

  (@collect $value:ident; [$($arms:tt)*] _ => $fallback:expr $(,)?) => {
    $crate::any_cast_match_mut!(@emit_fallback $value; [$($arms)*] $fallback)
  };
  (@collect $value:ident; [$($arms:tt)*] $source_type:ty => |$binding:ident| $body:expr, $($rest:tt)+) => {
    $crate::any_cast_match_mut!(@collect $value; [$($arms)* [$source_type][$binding][$body]] $($rest)+)
  };
  (@collect $value:ident; [$($arms:tt)*] $source_type:ty => |$binding:ident| $body:expr $(,)?) => {
    $crate::any_cast_match_mut!(@emit_option $value; [$($arms)* [$source_type][$binding][$body]])
  };

  (@emit_fallback $value:ident; [] $fallback:expr) => {
    $fallback
  };
  (@emit_fallback $value:ident; [[$source_type:ty][$binding:ident][$body:expr] $($rest:tt)*] $fallback:expr) => {{
    if let Some($binding) = $value.downcast_mut::<$source_type>() {
      $body
    } else {
      $crate::any_cast_match_mut!(@emit_fallback $value; [$($rest)*] $fallback)
    }
  }};

  (@emit_option $value:ident; [[$source_type:ty][$binding:ident][$body:expr] $($rest:tt)*]) => {{
    if let Some($binding) = $value.downcast_mut::<$source_type>() {
      ::core::option::Option::Some($body)
    } else {
      $crate::any_cast_match_mut!(@emit_option_rest $value; [$($rest)*])
    }
  }};
  (@emit_option_rest $value:ident; []) => {
    ::core::option::Option::None
  };
  (@emit_option_rest $value:ident; [[$source_type:ty][$binding:ident][$body:expr] $($rest:tt)*]) => {{
    if let Some($binding) = $value.downcast_mut::<$source_type>() {
      ::core::option::Option::Some($body)
    } else {
      $crate::any_cast_match_mut!(@emit_option_rest $value; [$($rest)*])
    }
  }};
}

#[cfg(test)]
mod tests {
  struct Cat {
    name: &'static str,
  }

  struct Dog {
    name: &'static str,
  }

  struct Wallet;

  #[test]
  fn any_cast_match_returns_some_for_first_matching_arm() {
    let cat = Cat { name: "mittens" };

    let name = any_cast_match!(&cat;
      Dog => |dog| dog.name,
      Cat => |cat| cat.name,
    );

    assert_eq!(name, Some("mittens"));
  }

  #[test]
  fn any_cast_match_returns_none_without_fallback() {
    let wallet = Wallet;

    let name = any_cast_match!(&wallet;
      Cat => |cat| cat.name,
      Dog => |dog| dog.name,
    );

    assert_eq!(name, None);
  }

  #[test]
  fn any_cast_match_uses_fallback_when_no_type_matches() {
    let wallet = Wallet;

    let name = any_cast_match!(&wallet;
      Cat => |cat| cat.name,
      Dog => |dog| dog.name,
      _ => "unknown",
    );

    assert_eq!(name, "unknown");
  }

  #[test]
  fn any_cast_match_mut_supports_mutable_arms() {
    let mut dog = Dog { name: "spot" };

    any_cast_match_mut!(&mut dog;
      Cat => |cat| cat.name = "kitty",
      Dog => |dog| dog.name = "buddy",
    )
    .unwrap();

    assert_eq!(dog.name, "buddy");
  }

  #[test]
  fn any_cast_match_mut_uses_fallback_when_no_type_matches() {
    let mut wallet = Wallet;

    let name = any_cast_match_mut!(&mut wallet;
      Cat => |cat| {
        cat.name = "kitty";
        "cat"
      },
      Dog => |dog| {
        dog.name = "buddy";
        "dog"
      },
      _ => "unknown",
    );

    assert_eq!(name, "unknown");
  }
}
