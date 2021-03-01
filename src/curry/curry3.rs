//! Author --- DMorgan  
//! Last Moddified --- 2021-03-01

use crate::*;
use core::ops::CoerceUnsized;

/// A curried ternary function.
/// 
/// ```rust
/// use curry::*;
/// 
/// let f = Curry3(|a, b, c| a + b + c);
/// assert_eq!(f(1, 2, 3), 6);
/// assert_eq!(f(1, 2)(3), 6);
/// assert_eq!(f(1)(2)(3), 6);
/// ```
#[repr(transparent,)]
#[derive(PartialEq, Eq, Clone, Copy, Default, Debug,)]
pub struct Curry3<F,>(pub F,)
  where F: ?Sized,;

impl<F,> Curry3<F,> {
  /// Constructs a new `Curry3` from `f`.
  #[inline]
  pub const fn new(f: F,) -> Self { Curry3(f,) }
  /// References the inner value.
  #[inline]
  pub const fn as_ref(&self,) -> Curry3<&F,> { Curry3(&self.0,) }
  /// Mutably references the inner value.
  #[inline]
  pub const fn as_mut(&mut self,) -> Curry3<&mut F,> { Curry3(&mut self.0,) }
}

impl<F,> Curry3<&'_ F,>
  where F: Clone, {
  /// Clones the inner value.
  #[inline]
  pub fn cloned(&self,) -> Curry3<F,> { Curry3(self.0.clone(),) }
}

impl<F,> Curry3<&'_ F,>
  where F: Copy, {
  /// Copies the inner value.
  #[inline]
  pub fn copied(&self,) -> Curry3<F,> { Curry3(*self.0,) }
}

impl<F,> Curry3<&'_ mut F,>
  where F: Clone, {
  /// Clones the inner value.
  #[inline]
  pub fn cloned(&self,) -> Curry3<F,> { Curry3(self.0.clone(),) }
}

impl<F,> Curry3<&'_ mut F,>
  where F: Copy, {
  /// Copies the inner value.
  #[inline]
  pub fn copied(&self,) -> Curry3<F,> { Curry3(*self.0,) }
}

impl<A, F,> FnOnce<(A,)> for Curry3<F,> {
  type Output = Closure2<A, Self,>;

  #[inline]
  extern "rust-call" fn call_once(self, (a,): (A,),) -> Self::Output { Closure2::new(a, self,) }
}

impl<A, F,> FnMut<(A,)> for Curry3<F,>
  where F: Clone, {
  extern "rust-call" fn call_mut(&mut self, (a,): (A,),) -> Self::Output { Closure2::new(a, self.clone(),) }
}

impl<A, F,> Fn<(A,)> for Curry3<F,>
  where F: Clone, {
  extern "rust-call" fn call(&self, (a,): (A,),) -> Self::Output { Closure2::new(a, self.clone(),) }
}

impl<A, B, F,> FnOnce<(A, B,)> for Curry3<F,> {
  type Output = Closure3<A, B, F,>;

  #[inline]
  extern "rust-call" fn call_once(self, (a, b,): (A, B,),) -> Self::Output { Closure3::new(a, b, self.0,) }
}

impl<A, B, F,> FnMut<(A, B,)> for Curry3<F,>
  where F: Clone, {
  extern "rust-call" fn call_mut(&mut self, (a, b,): (A, B,),) -> Self::Output { Closure3::new(a, b, self.0.clone(),) }
}

impl<A, B, F,> Fn<(A, B,)> for Curry3<F,>
  where F: Clone, {
  extern "rust-call" fn call(&self, (a, b,): (A, B,),) -> Self::Output { Closure3::new(a, b, self.0.clone(),) }
}

impl<A, B, C, F,> FnOnce<(A, B, C,)> for Curry3<F,>
  where F: FnOnce<(A, B, C,)>, {
  type Output = F::Output;

  #[inline]
  extern "rust-call" fn call_once(self, args: (A, B, C,),) -> Self::Output { self.0.call_once(args,) }
}

impl<A, B, C, F,> FnMut<(A, B, C,)> for Curry3<F,>
  where F: Fn<(A, B, C,)>, {
  #[inline]
  extern "rust-call" fn call_mut(&mut self, args: (A, B, C,),) -> Self::Output { self.0.call_mut(args,) }
}

impl<A, B, C, F,> Fn<(A, B, C,)> for Curry3<F,>
  where F: Fn<(A, B, C,)>, {
  #[inline]
  extern "rust-call" fn call(&self, args: (A, B, C,),) -> Self::Output { self.0.call(args,) }
}

impl<T, U,> CoerceUnsized<Curry3<U,>> for Curry3<T,>
  where T: CoerceUnsized<U> + ?Sized,
    U: ?Sized, {}

/// A closure making a ternary function a unary function.
/// 
/// ```rust
/// use curry::*;
/// 
/// let f: Closure3<_, _, _> = Curry3(|a, b, c| a + b + c)(1, 2);
/// assert_eq!(f(3), 6);
/// ```
#[derive(PartialEq, Eq, Clone, Copy, Default, Debug,)]
pub struct Closure3<A, B, F,>
  where F: ?Sized, {
  /// The stored `A` parameter.
  pub a: A,
  /// The stored `B` parameter.
  pub b: B,
  /// The function being wrapped.
  pub func: F,
}

impl<A, B, F,> Closure3<A, B, F,> {
  /// Constructs a new `Closure3` from `a`, `b` and `func`.
  #[inline]
  pub const fn new(a: A, b: B, func: F,) -> Self { Closure3 { a, b, func, } }
}

impl<A, B, C, F,> FnOnce<(C,)> for Closure3<A, B, F,>
  where F: FnOnce<(A, B, C,)>, {
  type Output = F::Output;

  #[inline]
  extern "rust-call" fn call_once(self, (c,): (C,),) -> Self::Output { (self.func)(self.a, self.b, c,) }
}

impl<A, B, C, F,> FnMut<(C,)> for Closure3<A, B, F,>
  where A: Clone,
    B: Clone,
    F: FnMut<(A, B, C,)>, {
  extern "rust-call" fn call_mut(&mut self, (c,): (C,),) -> Self::Output { (self.func)(self.a.clone(), self.b.clone(), c,) }
}

impl<A, B, C, F,> Fn<(C,)> for Closure3<A, B, F,>
  where A: Clone,
    B: Clone,
    F: Fn<(A, B, C,)>, {
  extern "rust-call" fn call(&self, (c,): (C,),) -> Self::Output { (self.func)(self.a.clone(), self.b.clone(), c,) }
}

impl<A, B, T, U,> CoerceUnsized<Closure3<A, B, U,>> for Closure3<A, B, T,>
  where T: CoerceUnsized<U> + ?Sized,
    U: ?Sized, {}

#[allow(unused,)]
fn _assert_coerce_unsized(a: Curry3<&i32,>, b: Closure3<(), (), &i32,>,) {
  let _: Curry3<&dyn Send,> = a;
  let _: Closure3<(), (), &dyn Send,> = b;
}
