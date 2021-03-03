//! Provides types for currying and uncurrying functions with up to 6 parameters.
//! 
//! Author --- DMorgan  
//! Last Moddified --- 2021-03-03

#![no_std]
#![deny(missing_docs,)]
#![feature(
  coerce_unsized, fn_traits, unboxed_closures, const_mut_refs, const_fn, external_doc,
)]
#![doc(test(attr(feature(fn_traits, unboxed_closures,),),),)]

#[cfg(test,)]
extern crate std;

mod curry {
  pub mod curry2;
  pub mod curry3;
  pub mod curry4;
  pub mod curry5;
  pub mod curry6;
}
mod uncurry;
mod fns_macros;

pub use self::{
  curry::{curry2::*, curry3::*, curry4::*, curry5::*, curry6::*,},
  uncurry::*,
  fns_macros::*,
};

#[cfg(doctest,)]
#[doc(include = "../README.md",)]
struct ReadmeDocs;
