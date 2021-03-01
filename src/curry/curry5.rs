//! Author --- DMorgan  
//! Last Moddified --- 2021-03-01

use crate::*;
use core::ops::CoerceUnsized;

/// A curried function of arity 5.
/// 
/// ```rust
/// use curry::*;
/// 
/// let f = Curry5(|a, b, c, d, e| a + b + c + d + e);
/// assert_eq!(f(1, 2, 3, 4, 5), 15);
/// assert_eq!(f(1, 2, 3, 4)(5), 15);
/// assert_eq!(f(1, 2, 3)(4)(5), 15);
/// assert_eq!(f(1, 2)(3)(4)(5), 15);
/// assert_eq!(f(1)(2)(3)(4)(5), 15);
/// ```
#[repr(transparent,)]
#[derive(PartialEq, Eq, Clone, Copy, Default, Debug,)]
pub struct Curry5<F,>(pub F,)
  where F: ?Sized,;

impl<F,> Curry5<F,> {
  /// Constructs a new `Curry4` from `f`.
  #[inline]
  pub const fn new(f: F,) -> Self { Curry5(f,) }
  /// References the inner value.
  #[inline]
  pub const fn as_ref(&self,) -> Curry5<&F,> { Curry5(&self.0,) }
  /// Mutably references the inner value.
  #[inline]
  pub const fn as_mut(&mut self,) -> Curry5<&mut F,> { Curry5(&mut self.0,) }
}

impl<F,> Curry5<&'_ F,>
  where F: Clone, {
  /// Clones the inner value.
  #[inline]
  pub fn cloned(&self,) -> Curry5<F,> { Curry5(self.0.clone(),) }
}

impl<F,> Curry5<&'_ F,>
  where F: Copy, {
  /// Copies the inner value.
  #[inline]
  pub fn copied(&self,) -> Curry5<F,> { Curry5(*self.0,) }
}

impl<F,> Curry5<&'_ mut F,>
  where F: Clone, {
  /// Clones the inner value.
  #[inline]
  pub fn cloned(&self,) -> Curry5<F,> { Curry5(self.0.clone(),) }
}

impl<F,> Curry5<&'_ mut F,>
  where F: Copy, {
  /// Copies the inner value.
  #[inline]
  pub fn copied(&self,) -> Curry5<F,> { Curry5(*self.0,) }
}

impl<A, F,> FnOnce<(A,)> for Curry5<F,>
  where F: Clone, {
  type Output = Closure2<A, Self,>;

  #[inline]
  extern "rust-call" fn call_once(self, (a,): (A,),) -> Self::Output { Closure2::new(a, self.clone(),) }
}

impl<A, F,> FnMut<(A,)> for Curry5<F,>
  where F: Clone, {
  extern "rust-call" fn call_mut(&mut self, (a,): (A,),) -> Self::Output { Closure2::new(a, self.clone(),) }
}

impl<A, F,> Fn<(A,)> for Curry5<F,>
  where F: Clone, {
  extern "rust-call" fn call(&self, (a,): (A,),) -> Self::Output { Closure2::new(a, self.clone(),) }
}

impl<A, B, F,> FnOnce<(A, B,)> for Curry5<F,>
  where F: Clone, {
  type Output = Closure3<A, B, Self,>;

  #[inline]
  extern "rust-call" fn call_once(self, (a, b,): (A, B,),) -> Self::Output { Closure3::new(a, b, self,) }
}

impl<A, B, F,> FnMut<(A, B,)> for Curry5<F,>
  where F: Clone, {
  extern "rust-call" fn call_mut(&mut self, (a, b,): (A, B,),) -> Self::Output { Closure3::new(a, b, self.clone(),) }
}

impl<A, B, F,> Fn<(A, B,)> for Curry5<F,>
  where F: Clone, {
  extern "rust-call" fn call(&self, (a, b,): (A, B,),) -> Self::Output { Closure3::new(a, b, self.clone(),) }
}

impl<A, B, C, F,> FnOnce<(A, B, C,)> for Curry5<F,> {
  type Output = Closure4<A, B, C, Self,>;

  #[inline]
  extern "rust-call" fn call_once(self, (a, b, c,): (A, B, C,),) -> Self::Output { Closure4::new(a, b, c, self,) }
}

impl<A, B, C, F,> FnMut<(A, B, C,)> for Curry5<F,>
  where F: Clone, {
  extern "rust-call" fn call_mut(&mut self, (a, b, c,): (A, B, C,),) -> Self::Output { Closure4::new(a, b, c, self.clone(),) }
}

impl<A, B, C, F,> Fn<(A, B, C,)> for Curry5<F,>
  where F: Clone, {
  extern "rust-call" fn call(&self, (a, b, c,): (A, B, C,),) -> Self::Output { Closure4::new(a, b, c, self.clone(),) }
}

impl<A, B, C, D, F,> FnOnce<(A, B, C, D,)> for Curry5<F,> {
  type Output = Closure5<A, B, C, D, F,>;

  #[inline]
  extern "rust-call" fn call_once(self, (a, b, c, d,): (A, B, C, D,),) -> Self::Output { Closure5::new(a, b, c, d, self.0,) }
}

impl<A, B, C, D, F,> FnMut<(A, B, C, D,)> for Curry5<F,>
  where F: Clone, {
  extern "rust-call" fn call_mut(&mut self, (a, b, c, d,): (A, B, C, D,),) -> Self::Output { Closure5::new(a, b, c, d, self.0.clone(),) }
}

impl<A, B, C, D, F,> Fn<(A, B, C, D,)> for Curry5<F,>
  where F: Clone, {
  extern "rust-call" fn call(&self, (a, b, c, d,): (A, B, C, D,),) -> Self::Output { Closure5::new(a, b, c, d, self.0.clone(),) }
}

impl<A, B, C, D, E, F,> FnOnce<(A, B, C, D, E,)> for Curry5<F,>
  where F: FnOnce<(A, B, C, D, E,)>, {
  type Output = F::Output;

  #[inline]
  extern "rust-call" fn call_once(self, args: (A, B, C, D, E,),) -> Self::Output { self.0.call_once(args,) }
}

impl<A, B, C, D, E, F,> FnMut<(A, B, C, D, E,)> for Curry5<F,>
  where F: Fn<(A, B, C, D, E,)>, {
  #[inline]
  extern "rust-call" fn call_mut(&mut self, args: (A, B, C, D, E,),) -> Self::Output { self.0.call_mut(args,) }
}

impl<A, B, C, D, E, F,> Fn<(A, B, C, D, E,)> for Curry5<F,>
  where F: Fn<(A, B, C, D, E,)>, {
  #[inline]
  extern "rust-call" fn call(&self, args: (A, B, C, D, E,),) -> Self::Output { self.0.call(args,) }
}

impl<T, U,> CoerceUnsized<Curry5<U,>> for Curry5<T,>
  where T: CoerceUnsized<U> + ?Sized,
    U: ?Sized, {}

/// A closure making a function of arity 5 a unary function.
/// 
/// ```rust
/// use curry::*;
/// 
/// let f: Closure5<_, _, _, _, _> = Curry5(|a, b, c, d, e| a + b + c + d + e)(1, 2, 3, 4);
/// assert_eq!(f(5), 15);
/// ```
#[derive(PartialEq, Eq, Clone, Copy, Default, Debug,)]
pub struct Closure5<A, B, C, D, F,>
  where F: ?Sized, {
  /// The stored `A` parameter.
  pub a: A,
  /// The stored `B` parameter.
  pub b: B,
  /// The stored `C` parameter.
  pub c: C,
  /// The stored `D` parameter.
  pub d: D,
  /// The function being wrapped.
  pub func: F,
}

impl<A, B, C, D, F,> Closure5<A, B, C, D, F,> {
  /// Constructs a new `Closure4` from `a`, `b`, `c`, `d`, and `func`.
  #[inline]
  pub const fn new(a: A, b: B, c: C, d: D, func: F,) -> Self { Closure5 { a, b, c, d, func, } }
}

impl<A, B, C, D, E, F,> FnOnce<(E,)> for Closure5<A, B, C, D, F,>
  where F: FnOnce<(A, B, C, D, E,)>, {
  type Output = F::Output;

  #[inline]
  extern "rust-call" fn call_once(self, (e,): (E,),) -> Self::Output { (self.func)(self.a, self.b, self.c, self.d, e,) }
}

impl<A, B, C, D, E, F,> FnMut<(E,)> for Closure5<A, B, C, D, F,>
  where A: Clone,
    B: Clone,
    C: Clone,
    D: Clone,
    F: FnMut<(A, B, C, D, E,)>, {
  extern "rust-call" fn call_mut(&mut self, (e,): (E,),) -> Self::Output {
    (self.func)(self.a.clone(), self.b.clone(), self.c.clone(), self.d.clone(), e,)
  }
}

impl<A, B, C, D, E, F,> Fn<(E,)> for Closure5<A, B, C, D, F,>
  where A: Clone,
    B: Clone,
    C: Clone,
    D: Clone,
    F: Fn<(A, B, C, D, E,)>, {
  extern "rust-call" fn call(&self, (e,): (E,),) -> Self::Output {
    (self.func)(self.a.clone(), self.b.clone(), self.c.clone(), self.d.clone(), e,)
  }
}

impl<A, B, C, D, T, U,> CoerceUnsized<Closure5<A, B, C, D, U,>> for Closure5<A, B, C, D, T,>
  where T: CoerceUnsized<U> + ?Sized,
    U: ?Sized, {}

#[allow(unused,)]
fn _assert_coerce_unsized(a: Curry5<&i32,>, b: Closure5<(), (), (), (), &i32,>,) {
  let _: Curry5<&dyn Send,> = a;
  let _: Closure5<(), (), (), (), &dyn Send,> = b;
}
