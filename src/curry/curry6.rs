//! Author --- DMorgan  
//! Last Moddified --- 2021-03-02

use crate::*;
use core::ops::CoerceUnsized;

/// A curried function of arity 6.
/// 
/// ```rust
/// use curry::*;
/// 
/// let f = Curry6(|a, b, c, d, e, f| a + b + c + d + e + f);
/// assert_eq!(f(1, 2, 3, 4, 5, 6), 21);
/// assert_eq!(f(1, 2, 3, 4, 5)(6), 21);
/// assert_eq!(f(1, 2, 3, 4)(5)(6), 21);
/// assert_eq!(f(1, 2, 3)(4)(5)(6), 21);
/// assert_eq!(f(1, 2)(3)(4)(5)(6), 21);
/// assert_eq!(f(1)(2)(3)(4)(5)(6), 21);
/// ```
#[repr(transparent,)]
#[derive(PartialEq, Eq, Clone, Copy, Default, Debug,)]
pub struct Curry6<F,>(pub F,)
  where F: ?Sized,;

impl<F,> Curry6<F,> {
  /// Constructs a new `Curry4` from `f`.
  #[inline]
  pub const fn new(f: F,) -> Self { Curry6(f,) }
  /// References the inner value.
  #[inline]
  pub const fn as_ref(&self,) -> Curry6<&F,> { Curry6(&self.0,) }
  /// Mutably references the inner value.
  #[inline]
  pub const fn as_mut(&mut self,) -> Curry6<&mut F,> { Curry6(&mut self.0,) }
}

impl<F,> Curry6<&'_ F,>
  where F: Clone, {
  /// Clones the inner value.
  #[inline]
  pub fn cloned(&self,) -> Curry6<F,> { Curry6(self.0.clone(),) }
}

impl<F,> Curry6<&'_ F,>
  where F: Copy, {
  /// Copies the inner value.
  #[inline]
  pub fn copied(&self,) -> Curry6<F,> { Curry6(*self.0,) }
}

impl<F,> Curry6<&'_ mut F,>
  where F: Clone, {
  /// Clones the inner value.
  #[inline]
  pub fn cloned(&self,) -> Curry6<F,> { Curry6(self.0.clone(),) }
}

impl<F,> Curry6<&'_ mut F,>
  where F: Copy, {
  /// Copies the inner value.
  #[inline]
  pub fn copied(&self,) -> Curry6<F,> { Curry6(*self.0,) }
}

impl<A, F,> FnOnce<(A,)> for Curry6<F,>
  where F: Clone, {
  type Output = Closure2<A, Self,>;

  #[inline]
  extern "rust-call" fn call_once(self, (a,): (A,),) -> Self::Output { Closure2::new(a, self.clone(),) }
}

impl<A, F,> FnMut<(A,)> for Curry6<F,>
  where F: Clone, {
  extern "rust-call" fn call_mut(&mut self, (a,): (A,),) -> Self::Output { Closure2::new(a, self.clone(),) }
}

impl<A, F,> Fn<(A,)> for Curry6<F,>
  where F: Clone, {
  extern "rust-call" fn call(&self, (a,): (A,),) -> Self::Output { Closure2::new(a, self.clone(),) }
}

impl<A, B, F,> FnOnce<(A, B,)> for Curry6<F,>
  where F: Clone, {
  type Output = Closure3<A, B, Self,>;

  #[inline]
  extern "rust-call" fn call_once(self, (a, b,): (A, B,),) -> Self::Output { Closure3::new(a, b, self,) }
}

impl<A, B, F,> FnMut<(A, B,)> for Curry6<F,>
  where F: Clone, {
  extern "rust-call" fn call_mut(&mut self, (a, b,): (A, B,),) -> Self::Output { Closure3::new(a, b, self.clone(),) }
}

impl<A, B, F,> Fn<(A, B,)> for Curry6<F,>
  where F: Clone, {
  extern "rust-call" fn call(&self, (a, b,): (A, B,),) -> Self::Output { Closure3::new(a, b, self.clone(),) }
}

impl<A, B, C, F,> FnOnce<(A, B, C,)> for Curry6<F,> {
  type Output = Closure4<A, B, C, Self,>;

  #[inline]
  extern "rust-call" fn call_once(self, (a, b, c,): (A, B, C,),) -> Self::Output { Closure4::new(a, b, c, self,) }
}

impl<A, B, C, F,> FnMut<(A, B, C,)> for Curry6<F,>
  where F: Clone, {
  extern "rust-call" fn call_mut(&mut self, (a, b, c,): (A, B, C,),) -> Self::Output { Closure4::new(a, b, c, self.clone(),) }
}

impl<A, B, C, F,> Fn<(A, B, C,)> for Curry6<F,>
  where F: Clone, {
  extern "rust-call" fn call(&self, (a, b, c,): (A, B, C,),) -> Self::Output { Closure4::new(a, b, c, self.clone(),) }
}

impl<A, B, C, D, F,> FnOnce<(A, B, C, D,)> for Curry6<F,> {
  type Output = Closure5<A, B, C, D, Self,>;

  #[inline]
  extern "rust-call" fn call_once(self, (a, b, c, d,): (A, B, C, D,),) -> Self::Output { Closure5::new(a, b, c, d, self,) }
}

impl<A, B, C, D, F,> FnMut<(A, B, C, D,)> for Curry6<F,>
  where F: Clone, {
  extern "rust-call" fn call_mut(&mut self, (a, b, c, d,): (A, B, C, D,),) -> Self::Output { Closure5::new(a, b, c, d, self.clone(),) }
}

impl<A, B, C, D, F,> Fn<(A, B, C, D,)> for Curry6<F,>
  where F: Clone, {
  extern "rust-call" fn call(&self, (a, b, c, d,): (A, B, C, D,),) -> Self::Output { Closure5::new(a, b, c, d, self.clone(),) }
}

impl<A, B, C, D, E, F,> FnOnce<(A, B, C, D, E,)> for Curry6<F,> {
  type Output = Closure6<A, B, C, D, E, F,>;

  #[inline]
  extern "rust-call" fn call_once(self, (a, b, c, d, e,): (A, B, C, D, E,),) -> Self::Output { Closure6::new(a, b, c, d, e, self.0,) }
}

impl<A, B, C, D, E, F,> FnMut<(A, B, C, D, E,)> for Curry6<F,>
  where F: Clone, {
  extern "rust-call" fn call_mut(&mut self, (a, b, c, d, e,): (A, B, C, D, E,),) -> Self::Output { Closure6::new(a, b, c, d, e, self.0.clone(),) }
}

impl<A, B, C, D, E, F,> Fn<(A, B, C, D, E,)> for Curry6<F,>
  where F: Clone, {
  extern "rust-call" fn call(&self, (a, b, c, d, e,): (A, B, C, D, E,),) -> Self::Output { Closure6::new(a, b, c, d, e, self.0.clone(),) }
}

impl<A, B, C, D, E, F, G,> FnOnce<(A, B, C, D, E, F,)> for Curry6<G,>
  where G: FnOnce<(A, B, C, D, E, F,)>, {
  type Output = G::Output;

  #[inline]
  extern "rust-call" fn call_once(self, args: (A, B, C, D, E, F,),) -> Self::Output { self.0.call_once(args,) }
}

impl<A, B, C, D, E, F, G,> FnMut<(A, B, C, D, E, F,)> for Curry6<G,>
  where G: Fn<(A, B, C, D, E, F,)>, {
  #[inline]
  extern "rust-call" fn call_mut(&mut self, args: (A, B, C, D, E, F,),) -> Self::Output { self.0.call_mut(args,) }
}

impl<A, B, C, D, E, F, G,> Fn<(A, B, C, D, E, F,)> for Curry6<G,>
  where G: Fn<(A, B, C, D, E, F,)>, {
  #[inline]
  extern "rust-call" fn call(&self, args: (A, B, C, D, E, F,),) -> Self::Output { self.0.call(args,) }
}

impl<T, U,> CoerceUnsized<Curry6<U,>> for Curry6<T,>
  where T: CoerceUnsized<U> + ?Sized,
    U: ?Sized, {}

/// A closure making a function of arity 6 a unary function.
/// 
/// ```rust
/// use curry::*;
/// 
/// let f: Closure6<_, _, _, _, _, _> = Curry6(|a, b, c, d, e, f| a + b + c + d + e + f)(1, 2, 3, 4, 5);
/// assert_eq!(f(6), 21);
/// ```
#[derive(PartialEq, Eq, Clone, Copy, Default, Debug,)]
pub struct Closure6<A, B, C, D, E, F,>
  where F: ?Sized, {
  /// The stored `A` parameter.
  pub a: A,
  /// The stored `B` parameter.
  pub b: B,
  /// The stored `C` parameter.
  pub c: C,
  /// The stored `D` parameter.
  pub d: D,
  /// The stored `E` parameter.
  pub e: E,
  /// The function being wrapped.
  pub func: F,
}

impl<A, B, C, D, E, F,> Closure6<A, B, C, D, E, F,> {
  /// Constructs a new `Closure4` from `a`, `b`, `c`, `d`, `e`, and `func`.
  #[inline]
  pub const fn new(a: A, b: B, c: C, d: D, e: E, func: F,) -> Self { Closure6 { a, b, c, d, e, func, } }
}

impl<A, B, C, D, E, F, G,> FnOnce<(F,)> for Closure6<A, B, C, D, E, G,>
  where G: FnOnce<(A, B, C, D, E, F,)>, {
  type Output = G::Output;

  #[inline]
  extern "rust-call" fn call_once(self, (f,): (F,),) -> Self::Output { (self.func)(self.a, self.b, self.c, self.d, self.e, f,) }
}

impl<A, B, C, D, E, F, G,> FnMut<(F,)> for Closure6<A, B, C, D, E, G,>
  where A: Clone,
    B: Clone,
    C: Clone,
    D: Clone,
    E: Clone,
    G: FnMut<(A, B, C, D, E, F,)>, {
  extern "rust-call" fn call_mut(&mut self, (f,): (F,),) -> Self::Output {
    (self.func)(self.a.clone(), self.b.clone(), self.c.clone(), self.d.clone(), self.e.clone(), f,)
  }
}

impl<A, B, C, D, E, F, G,> Fn<(F,)> for Closure6<A, B, C, D, E, G,>
  where A: Clone,
    B: Clone,
    C: Clone,
    D: Clone,
    E: Clone,
    G: Fn<(A, B, C, D, E, F,)>, {
  extern "rust-call" fn call(&self, (f,): (F,),) -> Self::Output {
    (self.func)(self.a.clone(), self.b.clone(), self.c.clone(), self.d.clone(), self.e.clone(), f,)
  }
}

impl<A, B, C, D, E, T, U,> CoerceUnsized<Closure6<A, B, C, D, E, U,>> for Closure6<A, B, C, D, E, T,>
  where T: CoerceUnsized<U> + ?Sized,
    U: ?Sized, {}

#[allow(unused,)]
fn _assert_coerce_unsized(a: Curry6<&i32,>, b: Closure6<(), (), (), (), (), &i32,>,) {
  let _: Curry6<&dyn Send,> = a;
  let _: Closure6<(), (), (), (), (), &dyn Send,> = b;
}
