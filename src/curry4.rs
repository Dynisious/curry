//! Author --- DMorgan  
//! Last Moddified --- 2021-03-01

use super::*;
use core::ops::CoerceUnsized;

/// A curried binary function.
/// 
/// ```rust
/// use curry::*;
/// 
/// let f = Curry4(|a, b, c, d| a + b + c + d);
/// assert_eq!(f(1, 2, 3, 4), 10);
/// assert_eq!(f(1, 2, 3)(4), 10);
/// assert_eq!(f(1)(2)(3)(4), 10);
/// ```
#[repr(transparent,)]
#[derive(PartialEq, Eq, Clone, Copy, Default, Debug,)]
pub struct Curry4<F,>(pub F,)
  where F: ?Sized,;

impl<F,> Curry4<F,> {
  /// Constructs a new `Curry4` from `f`.
  #[inline]
  pub const fn new(f: F,) -> Self { Curry4(f,) }
  /// References the inner value.
  #[inline]
  pub const fn as_ref(&self,) -> Curry4<&F,> { Curry4(&self.0,) }
  /// Mutably references the inner value.
  #[inline]
  pub const fn as_mut(&mut self,) -> Curry4<&mut F,> { Curry4(&mut self.0,) }
}

impl<F,> Curry4<&'_ F,>
  where F: Clone, {
  /// Clones the inner value.
  #[inline]
  pub fn cloned(&self,) -> Curry4<F,> { Curry4(self.0.clone(),) }
}

impl<F,> Curry4<&'_ F,>
  where F: Copy, {
  /// Copies the inner value.
  #[inline]
  pub fn copied(&self,) -> Curry4<F,> { Curry4(*self.0,) }
}

impl<F,> Curry4<&'_ mut F,>
  where F: Clone, {
  /// Clones the inner value.
  #[inline]
  pub fn cloned(&self,) -> Curry4<F,> { Curry4(self.0.clone(),) }
}

impl<F,> Curry4<&'_ mut F,>
  where F: Copy, {
  /// Copies the inner value.
  #[inline]
  pub fn copied(&self,) -> Curry4<F,> { Curry4(*self.0,) }
}

impl<A, F,> FnOnce<(A,)> for Curry4<F,>
  where F: Clone, {
  type Output = Closure2<A, Self,>;

  #[inline]
  extern "rust-call" fn call_once(self, (a,): (A,),) -> Self::Output { Closure2::new(a, self.clone(),) }
}

impl<A, F,> FnMut<(A,)> for Curry4<F,>
  where F: Clone, {
  extern "rust-call" fn call_mut(&mut self, (a,): (A,),) -> Self::Output { Closure2::new(a, self.clone(),) }
}

impl<A, F,> Fn<(A,)> for Curry4<F,>
  where F: Clone, {
  extern "rust-call" fn call(&self, (a,): (A,),) -> Self::Output { Closure2::new(a, self.clone(),) }
}

impl<A, B, F,> FnOnce<(A, B,)> for Curry4<F,>
  where F: Clone, {
  type Output = Closure3<A, B, Self,>;

  #[inline]
  extern "rust-call" fn call_once(self, (a, b,): (A, B,),) -> Self::Output { Closure3::new(a, b, self,) }
}

impl<A, B, F,> FnMut<(A, B,)> for Curry4<F,>
  where F: Clone, {
  extern "rust-call" fn call_mut(&mut self, (a, b,): (A, B,),) -> Self::Output { Closure3::new(a, b, self.clone(),) }
}

impl<A, B, F,> Fn<(A, B,)> for Curry4<F,>
  where F: Clone, {
  extern "rust-call" fn call(&self, (a, b,): (A, B,),) -> Self::Output { Closure3::new(a, b, self.clone(),) }
}

impl<A, B, C, F,> FnOnce<(A, B, C,)> for Curry4<F,> {
  type Output = Closure4<A, B, C, F,>;

  #[inline]
  extern "rust-call" fn call_once(self, (a, b, c,): (A, B, C,),) -> Self::Output { Closure4::new(a, b, c, self.0,) }
}

impl<A, B, C, F,> FnMut<(A, B, C,)> for Curry4<F,>
  where F: Clone, {
  extern "rust-call" fn call_mut(&mut self, (a, b, c,): (A, B, C,),) -> Self::Output { Closure4::new(a, b, c, self.0.clone(),) }
}

impl<A, B, C, F,> Fn<(A, B, C,)> for Curry4<F,>
  where F: Clone, {
  extern "rust-call" fn call(&self, (a, b, c,): (A, B, C,),) -> Self::Output { Closure4::new(a, b, c, self.0.clone(),) }
}

impl<A, B, C, D, F,> FnOnce<(A, B, C, D,)> for Curry4<F,>
  where F: FnOnce<(A, B, C, D,)>, {
  type Output = F::Output;

  #[inline]
  extern "rust-call" fn call_once(self, args: (A, B, C, D,),) -> Self::Output { self.0.call_once(args,) }
}

impl<A, B, C, D, F,> FnMut<(A, B, C, D,)> for Curry4<F,>
  where F: Fn<(A, B, C, D,)>, {
  #[inline]
  extern "rust-call" fn call_mut(&mut self, args: (A, B, C, D,),) -> Self::Output { self.0.call_mut(args,) }
}

impl<A, B, C, D, F,> Fn<(A, B, C, D,)> for Curry4<F,>
  where F: Fn<(A, B, C, D,)>, {
  #[inline]
  extern "rust-call" fn call(&self, args: (A, B, C, D,),) -> Self::Output { self.0.call(args,) }
}

impl<T, U,> CoerceUnsized<Curry4<U,>> for Curry4<T,>
  where T: CoerceUnsized<U> + ?Sized,
    U: ?Sized, {}

/// A closure making a quaternary function a unary function.
/// 
/// ```rust
/// use curry::*;
/// 
/// let f: Closure4<_, _, _, _> = Curry4(|a, b, c, d| a + b + c + d)(1, 2, 3);
/// assert_eq!(f(4), 10);
/// ```
#[derive(PartialEq, Eq, Clone, Copy, Default, Debug,)]
pub struct Closure4<A, B, C, F,>
  where F: ?Sized, {
  /// The stored `A` parameter.
  pub a: A,
  /// The stored `B` parameter.
  pub b: B,
  /// The stored `C` parameter.
  pub c: C,
  /// The function being wrapped.
  pub func: F,
}

impl<A, B, C, F,> Closure4<A, B, C, F,> {
  /// Constructs a new `Closure4` from `a`, `b`, `c`, and `func`.
  #[inline]
  pub const fn new(a: A, b: B, c: C, func: F,) -> Self { Closure4 { a, b, c, func, } }
}

impl<A, B, C, D, F,> FnOnce<(D,)> for Closure4<A, B, C, F,>
  where F: FnOnce<(A, B, C, D,)>, {
  type Output = F::Output;

  #[inline]
  extern "rust-call" fn call_once(self, (d,): (D,),) -> Self::Output { (self.func)(self.a, self.b, self.c, d,) }
}

impl<A, B, C, D, F,> FnMut<(D,)> for Closure4<A, B, C, F,>
  where A: Clone,
    B: Clone,
    C: Clone,
    F: FnMut<(A, B, C, D,)>, {
  extern "rust-call" fn call_mut(&mut self, (d,): (D,),) -> Self::Output { (self.func)(self.a.clone(), self.b.clone(), self.c.clone(), d,) }
}

impl<A, B, C, D, F,> Fn<(D,)> for Closure4<A, B, C, F,>
  where A: Clone,
    B: Clone,
    C: Clone,
    F: Fn<(A, B, C, D,)>, {
  extern "rust-call" fn call(&self, (d,): (D,),) -> Self::Output { (self.func)(self.a.clone(), self.b.clone(), self.c.clone(), d,) }
}

impl<A, B, C, T, U,> CoerceUnsized<Closure4<A, B, C, U,>> for Closure4<A, B, C, T,>
  where T: CoerceUnsized<U> + ?Sized,
    U: ?Sized, {}

#[allow(unused,)]
fn _assert_coerce_unsized(a: Curry4<&i32,>, b: Closure4<(), (), (), &i32,>,) {
  let _: Curry4<&dyn Send,> = a;
  let _: Closure4<(), (), (), &dyn Send,> = b;
}
