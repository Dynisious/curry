//! Author --- DMorgan  
//! Last Moddified --- 2021-03-01

use core::ops::CoerceUnsized;

/// An uncurried function.
/// 
/// ```rust
/// use curry::*;
/// 
/// let f = Uncurry(|a| move |b| move |c| move |d| move |e| move |f| a + b + c + d + e + f);
/// assert_eq!(f(1, 2, 3, 4, 5, 6), 21);
/// assert_eq!(f(1, 2, 3, 4, 5)(6), 21);
/// assert_eq!(f(1, 2, 3, 4)(5)(6), 21);
/// assert_eq!(f(1, 2, 3)(4)(5)(6), 21);
/// assert_eq!(f(1, 2)(3)(4)(5)(6), 21);
/// assert_eq!(f(1)(2)(3)(4)(5)(6), 21);
/// ```
#[repr(transparent,)]
#[derive(PartialEq, Eq, Clone, Copy, Default, Debug,)]
pub struct Uncurry<F,>(pub F,)
  where F: ?Sized,;

impl<A, F,> FnOnce<(A,)> for Uncurry<F,>
  where F: FnOnce<(A,)>, {
  type Output = F::Output;

  extern "rust-call" fn call_once(self, args: (A,),) -> Self::Output { self.0.call_once(args,) }
}

impl<A, F,> FnMut<(A,)> for Uncurry<F,>
  where F: FnMut<(A,)>, {
  extern "rust-call" fn call_mut(&mut self, args: (A,),) -> Self::Output { self.0.call_mut(args,) }
}

impl<A, F,> Fn<(A,)> for Uncurry<F,>
  where F: Fn<(A,)>, {
  extern "rust-call" fn call(&self, args: (A,),) -> Self::Output { self.0.call(args,) }
}

impl<A, B, F,> FnOnce<(A, B,)> for Uncurry<F,>
  where F: FnOnce<(A,)>,
    F::Output: FnOnce<(B,)>, {
  type Output = <F::Output as FnOnce<(B,)>>::Output;

  extern "rust-call" fn call_once(self, (a, b,): (A, B,),) -> Self::Output { (self.0)(a,)(b,) }
}

impl<A, B, F,> FnMut<(A, B,)> for Uncurry<F,>
  where F: FnMut<(A,)>,
    F::Output: FnOnce<(B,)>, {
  extern "rust-call" fn call_mut(&mut self, (a, b,): (A, B,),) -> Self::Output { (self.0)(a,)(b,) }
}

impl<A, B, F,> Fn<(A, B,)> for Uncurry<F,>
  where F: Fn<(A,)>,
    F::Output: FnOnce<(B,)>, {
  extern "rust-call" fn call(&self, (a, b,): (A, B,),) -> Self::Output { (self.0)(a,)(b,) }
}

impl<A, B, C, F,> FnOnce<(A, B, C,)> for Uncurry<F,>
  where F: FnOnce<(A,)>,
    F::Output: FnOnce<(B,)>,
    <F::Output as FnOnce<(B,)>>::Output: FnOnce<(C,)>, {
  type Output = <<F::Output as FnOnce<(B,)>>::Output as FnOnce<(C,)>>::Output;

  extern "rust-call" fn call_once(self, (a, b, c,): (A, B, C,),) -> Self::Output { (self.0)(a,)(b,)(c,) }
}

impl<A, B, C, F,> FnMut<(A, B, C,)> for Uncurry<F,>
  where F: FnMut<(A,)>,
    F::Output: FnOnce<(B,)>,
    <F::Output as FnOnce<(B,)>>::Output: FnOnce<(C,)>, {
  extern "rust-call" fn call_mut(&mut self, (a, b, c,): (A, B, C,),) -> Self::Output { (self.0)(a,)(b,)(c,) }
}

impl<A, B, C, F,> Fn<(A, B, C,)> for Uncurry<F,>
  where F: Fn<(A,)>,
    F::Output: FnOnce<(B,)>,
    <F::Output as FnOnce<(B,)>>::Output: FnOnce<(C,)>, {
  extern "rust-call" fn call(&self, (a, b, c,): (A, B, C,),) -> Self::Output { (self.0)(a,)(b,)(c,) }
}

impl<A, B, C, D, F,> FnOnce<(A, B, C, D,)> for Uncurry<F,>
  where F: FnOnce<(A,)>,
    F::Output: FnOnce<(B,)>,
    <F::Output as FnOnce<(B,)>>::Output: FnOnce<(C,)>,
    <<F::Output as FnOnce<(B,)>>::Output as FnOnce<(C,)>>::Output: FnOnce<(D,)>, {
  type Output = <<<F::Output as FnOnce<(B,)>>::Output as FnOnce<(C,)>>::Output as FnOnce<(D,)>>::Output;

  extern "rust-call" fn call_once(self, (a, b, c, d,): (A, B, C, D,),) -> Self::Output { (self.0)(a,)(b,)(c,)(d,) }
}

impl<A, B, C, D, F,> FnMut<(A, B, C, D,)> for Uncurry<F,>
  where F: FnMut<(A,)>,
    F::Output: FnOnce<(B,)>,
    <F::Output as FnOnce<(B,)>>::Output: FnOnce<(C,)>,
    <<F::Output as FnOnce<(B,)>>::Output as FnOnce<(C,)>>::Output: FnOnce<(D,)>, {
  extern "rust-call" fn call_mut(&mut self, (a, b, c, d,): (A, B, C, D,),) -> Self::Output { (self.0)(a,)(b,)(c,)(d,) }
}

impl<A, B, C, D, F,> Fn<(A, B, C, D,)> for Uncurry<F,>
  where F: Fn<(A,)>,
    F::Output: FnOnce<(B,)>,
    <F::Output as FnOnce<(B,)>>::Output: FnOnce<(C,)>,
    <<F::Output as FnOnce<(B,)>>::Output as FnOnce<(C,)>>::Output: FnOnce<(D,)>, {
  extern "rust-call" fn call(&self, (a, b, c, d,): (A, B, C, D,),) -> Self::Output { (self.0)(a,)(b,)(c,)(d,) }
}

impl<A, B, C, D, E, F,> FnOnce<(A, B, C, D, E,)> for Uncurry<F,>
  where F: FnOnce<(A,)>,
    F::Output: FnOnce<(B,)>,
    <F::Output as FnOnce<(B,)>>::Output: FnOnce<(C,)>,
    <<F::Output as FnOnce<(B,)>>::Output as FnOnce<(C,)>>::Output: FnOnce<(D,)>,
    <<<F::Output as FnOnce<(B,)>>::Output as FnOnce<(C,)>>::Output as FnOnce<(D,)>>::Output: FnOnce<(E,)>, {
  type Output = <<<<F::Output as FnOnce<(B,)>>::Output as FnOnce<(C,)>>::Output as FnOnce<(D,)>>::Output as FnOnce<(E,)>>::Output;

  extern "rust-call" fn call_once(self, (a, b, c, d, e,): (A, B, C, D, E,),) -> Self::Output { (self.0)(a,)(b,)(c,)(d,)(e,) }
}

impl<A, B, C, D, E, F,> FnMut<(A, B, C, D, E,)> for Uncurry<F,>
  where F: FnMut<(A,)>,
    F::Output: FnOnce<(B,)>,
    <F::Output as FnOnce<(B,)>>::Output: FnOnce<(C,)>,
    <<F::Output as FnOnce<(B,)>>::Output as FnOnce<(C,)>>::Output: FnOnce<(D,)>,
    <<<F::Output as FnOnce<(B,)>>::Output as FnOnce<(C,)>>::Output as FnOnce<(D,)>>::Output: FnOnce<(E,)>, {
  extern "rust-call" fn call_mut(&mut self, (a, b, c, d, e,): (A, B, C, D, E,),) -> Self::Output { (self.0)(a,)(b,)(c,)(d,)(e,) }
}

impl<A, B, C, D, E, F,> Fn<(A, B, C, D, E,)> for Uncurry<F,>
  where F: Fn<(A,)>,
    F::Output: FnOnce<(B,)>,
    <F::Output as FnOnce<(B,)>>::Output: FnOnce<(C,)>,
    <<F::Output as FnOnce<(B,)>>::Output as FnOnce<(C,)>>::Output: FnOnce<(D,)>,
    <<<F::Output as FnOnce<(B,)>>::Output as FnOnce<(C,)>>::Output as FnOnce<(D,)>>::Output: FnOnce<(E,)>, {
  extern "rust-call" fn call(&self, (a, b, c, d, e,): (A, B, C, D, E,),) -> Self::Output { (self.0)(a,)(b,)(c,)(d,)(e,) }
}

impl<A, B, C, D, E, F, G,> FnOnce<(A, B, C, D, E, F,)> for Uncurry<G,>
  where G: FnOnce<(A,)>,
    G::Output: FnOnce<(B,)>,
    <G::Output as FnOnce<(B,)>>::Output: FnOnce<(C,)>,
    <<G::Output as FnOnce<(B,)>>::Output as FnOnce<(C,)>>::Output: FnOnce<(D,)>,
    <<<G::Output as FnOnce<(B,)>>::Output as FnOnce<(C,)>>::Output as FnOnce<(D,)>>::Output: FnOnce<(E,)>,
    <<<<G::Output as FnOnce<(B,)>>::Output as FnOnce<(C,)>>::Output as FnOnce<(D,)>>::Output as FnOnce<(E,)>>::Output: FnOnce<(F,)>, {
  type Output = <<<<<G::Output as FnOnce<(B,)>>::Output as FnOnce<(C,)>>::Output as FnOnce<(D,)>>::Output as FnOnce<(E,)>>::Output as FnOnce<(F,)>>::Output;

  extern "rust-call" fn call_once(self, (a, b, c, d, e, f,): (A, B, C, D, E, F,),) -> Self::Output { (self.0)(a,)(b,)(c,)(d,)(e,)(f,) }
}

impl<A, B, C, D, E, F, G,> FnMut<(A, B, C, D, E, F,)> for Uncurry<G,>
  where G: FnMut<(A,)>,
    G::Output: FnOnce<(B,)>,
    <G::Output as FnOnce<(B,)>>::Output: FnOnce<(C,)>,
    <<G::Output as FnOnce<(B,)>>::Output as FnOnce<(C,)>>::Output: FnOnce<(D,)>,
    <<<G::Output as FnOnce<(B,)>>::Output as FnOnce<(C,)>>::Output as FnOnce<(D,)>>::Output: FnOnce<(E,)>,
    <<<<G::Output as FnOnce<(B,)>>::Output as FnOnce<(C,)>>::Output as FnOnce<(D,)>>::Output as FnOnce<(E,)>>::Output: FnOnce<(F,)>, {
  extern "rust-call" fn call_mut(&mut self, (a, b, c, d, e, f,): (A, B, C, D, E, F,),) -> Self::Output { (self.0)(a,)(b,)(c,)(d,)(e,)(f,) }
}

impl<A, B, C, D, E, F, G,> Fn<(A, B, C, D, E, F,)> for Uncurry<G,>
  where G: Fn<(A,)>,
    G::Output: FnOnce<(B,)>,
    <G::Output as FnOnce<(B,)>>::Output: FnOnce<(C,)>,
    <<G::Output as FnOnce<(B,)>>::Output as FnOnce<(C,)>>::Output: FnOnce<(D,)>,
    <<<G::Output as FnOnce<(B,)>>::Output as FnOnce<(C,)>>::Output as FnOnce<(D,)>>::Output: FnOnce<(E,)>,
    <<<<G::Output as FnOnce<(B,)>>::Output as FnOnce<(C,)>>::Output as FnOnce<(D,)>>::Output as FnOnce<(E,)>>::Output: FnOnce<(F,)>, {
  extern "rust-call" fn call(&self, (a, b, c, d, e, f,): (A, B, C, D, E, F,),) -> Self::Output { (self.0)(a,)(b,)(c,)(d,)(e,)(f,) }
}

impl<T, U,> CoerceUnsized<Uncurry<U,>> for Uncurry<T,>
  where T: CoerceUnsized<U> + ?Sized,
    U: ?Sized, {}

#[allow(unused,)]
fn _assert_coerce_unsized(a: Uncurry<&i32,>,) {
  let _: Uncurry<&dyn Send,> = a;
}