
# Curry

Provides types for currying and uncurrying functions with up to 6 parameters.

```rust
use curry::*;

let f = Curry6(|a, b, c, d, e, f| a + b + c + d + e + f);
assert_eq!(f(1, 2, 3, 4, 5, 6), 21);
assert_eq!(f(1, 2, 3, 4, 5)(6), 21);
assert_eq!(f(1, 2, 3, 4)(5)(6), 21);
assert_eq!(f(1, 2, 3)(4)(5)(6), 21);
assert_eq!(f(1, 2)(3)(4)(5)(6), 21);
assert_eq!(f(1)(2)(3)(4)(5)(6), 21);
```

```rust
use curry::*;

let f = Uncurry(|a| move |b| move |c| move |d| move |e| move |f| a + b + c + d + e + f);
assert_eq!(f(1, 2, 3, 4, 5, 6), 21);
assert_eq!(f(1, 2, 3, 4, 5)(6), 21);
assert_eq!(f(1, 2, 3, 4)(5)(6), 21);
assert_eq!(f(1, 2, 3)(4)(5)(6), 21);
assert_eq!(f(1, 2)(3)(4)(5)(6), 21);
assert_eq!(f(1)(2)(3)(4)(5)(6), 21);
```

## [`impl_fn`](self::impl_fn)

A convenience macro for implementing the `Fn*` family of traits on a type.

```rust
use curry::*;

struct FnT;

impl_fn!(<A> Fn(A,) for FnT => (_) -> i32 { 42 });
impl_fn!(<'a, A, B> Fn(&'a A, A, B,) for FnT => (a1, a2, b) -> (&'a A, B, A) { (a1, b, a2) });

assert_eq!((FnT)('c'), 42);
assert_eq!((FnT)(&1, 2, 3), (&1, 3, 2));
```
