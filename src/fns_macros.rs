//! Author --- DMorgan  
//! Last Moddified --- 2021-03-03

/// A convenience macro for implementing `Fn*` traits for types.
/// 
/// For macro reasons the tuple of argument types must have a trailing comma.
/// 
/// # Examples
/// 
/// ```rust
/// use curry::*;
/// 
/// struct Unit;
/// 
/// impl_fn!(Fn() for Unit => () -> () {});
/// 
/// struct Id;
/// 
/// impl_fn!(Fn(i32,) for Id => (x) -> i32 { x });
/// 
/// struct Generics;
/// 
/// impl_fn!(<A> Fn(A,) for Generics => (_) -> i32 { 42 });
/// impl_fn!(<'a, A, B> Fn(&'a A, A, B,) for Generics => (a1, a2, b) -> (&'a A, B, A) { (a1, b, a2) });
/// 
/// assert_eq!((Unit)(), ());
/// assert_eq!((Id)(42), 42);
/// assert_eq!((Generics)('c'), 42);
/// assert_eq!((Generics)(&1, 2, 3), (&1, 3, 2));
/// ```
#[macro_export(local_inner_macros,)]
macro_rules! impl_fn {
  ($(<$($life:lifetime,)* $($gen:ident),+ $(,)?>)? Fn ($($arg_tp:ty,)*) for $type:ty => ($($arg:pat),* $(,)?) -> $ret:ty $body:block) => {
    impl $(<$($life,)* $($gen),+>)? FnOnce<($($arg_tp,)*)> for $type {
      type Output = $ret;

      #[inline(always,)]
      extern "rust-call" fn call_once(self, args : ($($arg_tp,)*),) -> Self::Output { Fn::call(&self, args,) }
    }

    impl $(<$($life,)* $($gen),+>)? FnMut<($($arg_tp,)*)> for $type {
      #[inline(always,)]
      extern "rust-call" fn call_mut(&mut self, args : ($($arg_tp,)*),) -> Self::Output { Fn::call(self, args,) }
    }

    impl $(<$($life,)* $($gen),+>)? Fn<($($arg_tp,)*)> for $type {
      extern "rust-call" fn call(&self, ($($arg,)*) : ($($arg_tp,)*),) -> Self::Output $body
    }
  };
}

#[cfg(test,)]
mod tests {
  struct AssertFn;

  impl_fn!(Fn() for AssertFn => () -> () {});

  impl_fn!(Fn(bool, i32,) for AssertFn => (b, i,) -> bool { b || i == 42 });

  impl_fn!(<'a, 'b, A, B,> Fn(&'a bool, &'b mut A, B,) for AssertFn => (a, _, b,) -> B { assert!(*a); b });

  #[test]
  fn test_impls() {
    assert_eq!(AssertFn(), ());
    assert_eq!(AssertFn(false, 42,), true);
    assert_eq!(AssertFn(&true, &mut 0, 'c'), 'c');
  }
}
