//! Provides types for currying functions with up to 5 parameters.
//! 
//! Author --- DMorgan  
//! Last Moddified --- 2021-03-01

#![no_std]
#![deny(missing_docs,)]
#![feature(
  coerce_unsized, fn_traits, unboxed_closures, const_mut_refs, const_fn, external_doc,
)]

#[cfg(test,)]
extern crate std;

mod curry2;
mod curry3;
mod curry4;
mod curry5;

pub use self::{curry2::*, curry3::*, curry4::*, curry5::*,};

#[cfg(doctest,)]
#[doc(include = "../README.md",)]
struct ReadmeDocs;
