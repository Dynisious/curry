//! Author --- DMorgan  
//! Last Moddified --- 2021-03-01

use core::ops::CoerceUnsized;

/// A curried binary function.
/// 
/// ```rust
/// use curry::*;
/// 
/// let f = Curry2(|a, b| a + b);
/// assert_eq!(f(1, 2), 3);
/// assert_eq!(f(1)(2), 3);
/// ```
#[repr(transparent,)]
#[derive(PartialEq, Eq, Clone, Copy, Default, Debug,)]
pub struct Curry2<F,>(pub F,)
  where F: ?Sized,;

impl<F,> Curry2<F,> {
  /// Constructs a new `Curry2` from `f`.
  #[inline]
  pub const fn new(f: F,) -> Self { Curry2(f,) }
  /// References the inner value.
  #[inline]
  pub const fn as_ref(&self,) -> Curry2<&F,> { Curry2(&self.0,) }
  /// Mutably references the inner value.
  #[inline]
  pub const fn as_mut(&mut self,) -> Curry2<&mut F,> { Curry2(&mut self.0,) }
}

impl<F,> Curry2<&'_ F,>
  where F: Clone, {
  /// Clones the inner value.
  #[inline]
  pub fn cloned(&self,) -> Curry2<F,> { Curry2(self.0.clone(),) }
}

impl<F,> Curry2<&'_ F,>
  where F: Copy, {
  /// Copies the inner value.
  #[inline]
  pub fn copied(&self,) -> Curry2<F,> { Curry2(*self.0,) }
}

impl<F,> Curry2<&'_ mut F,>
  where F: Clone, {
  /// Clones the inner value.
  #[inline]
  pub fn cloned(&self,) -> Curry2<F,> { Curry2(self.0.clone(),) }
}

impl<F,> Curry2<&'_ mut F,>
  where F: Copy, {
  /// Copies the inner value.
  #[inline]
  pub fn copied(&self,) -> Curry2<F,> { Curry2(*self.0,) }
}

impl<A, F,> FnOnce<(A,)> for Curry2<F,> {
  type Output = Closure2<A, F,>;

  #[inline]
  extern "rust-call" fn call_once(self, (a,): (A,),) -> Self::Output { Closure2::new(a, self.0,) }
}

impl<A, F,> FnMut<(A,)> for Curry2<F,>
  where F: Clone, {
  extern "rust-call" fn call_mut(&mut self, (a,): (A,),) -> Self::Output { Closure2::new(a, self.0.clone(),) }
}

impl<A, F,> Fn<(A,)> for Curry2<F,>
  where F: Clone, {
  extern "rust-call" fn call(&self, (a,): (A,),) -> Self::Output { Closure2::new(a, self.0.clone(),) }
}

impl<A, B, F,> FnOnce<(A, B,)> for Curry2<F,>
  where F: FnOnce<(A, B,)> {
  type Output = F::Output;

  #[inline]
  extern "rust-call" fn call_once(self, args: (A, B,),) -> Self::Output { self.0.call_once(args,) }
}

impl<A, B, F,> FnMut<(A, B,)> for Curry2<F,>
  where F: FnMut<(A, B,)>, {
  #[inline]
  extern "rust-call" fn call_mut(&mut self, args: (A, B,),) -> Self::Output { self.0.call_mut(args,) }
}

impl<A, B, F,> Fn<(A, B,)> for Curry2<F,>
  where F: Fn<(A, B,)>, {
  #[inline]
  extern "rust-call" fn call(&self, args: (A, B,),) -> Self::Output { self.0.call(args,) }
}

impl<T, U,> CoerceUnsized<Curry2<U,>> for Curry2<T,>
  where T: CoerceUnsized<U> + ?Sized,
    U: ?Sized, {}

/// A closure making a binary function into a curried function.
/// 
/// ```rust
/// use curry::*;
/// 
/// let f: Closure2<_, _> = Curry2(|a, b| a + b)(1);
/// assert_eq!(f(2), 3);
/// ```
#[derive(PartialEq, Eq, Clone, Copy, Default, Debug,)]
pub struct Closure2<A, F,>
  where F: ?Sized, {
  /// The stored `A` parameter.
  pub a: A,
  /// The function being wrapped.
  pub func: F,
}

impl<A, F,> Closure2<A, F,> {
  /// Constructs a new `Closure2` from `a`, and `func`.
  #[inline]
  pub const fn new(a: A, func: F,) -> Self { Closure2 { a, func, } }
}

impl<A, B, F,> FnOnce<(B,)> for Closure2<A, F,>
  where F: FnOnce<(A, B,)>, {
  type Output = F::Output;

  #[inline]
  extern "rust-call" fn call_once(self, (b,): (B,),) -> Self::Output { (self.func)(self.a, b,) }
}

impl<A, B, F,> FnMut<(B,)> for Closure2<A, F,>
  where A: Clone,
    F: FnMut<(A, B,)>, {
  extern "rust-call" fn call_mut(&mut self, (b,): (B,),) -> Self::Output { (self.func)(self.a.clone(), b,) }
}

impl<A, B, F,> Fn<(B,)> for Closure2<A, F,>
  where A: Clone,
    F: Fn<(A, B,)>, {
  extern "rust-call" fn call(&self, (b,): (B,),) -> Self::Output { (self.func)(self.a.clone(), b,) }
}

impl<A, T, U,> CoerceUnsized<Closure2<A, U,>> for Closure2<A, T,>
  where T: CoerceUnsized<U> + ?Sized,
    U: ?Sized, {}

#[allow(unused,)]
fn _assert_coerce_unsized(a: Curry2<&i32,>, b: Closure2<(), &i32,>,) {
  let _: Curry2<&dyn Send,> = a;
  let _: Closure2<(), &dyn Send,> = b;
}
