
# Curry

Provides types for currying and uncurrying functions with up to 5 parameters.

```rust
use curry::*;

let f = Curry5(|a, b, c, d, e| a + b + c + d + e);
assert_eq!(f(1, 2, 3, 4, 5), 15);
assert_eq!(f(1, 2, 3, 4)(5), 15);
assert_eq!(f(1, 2, 3)(4)(5), 15);
assert_eq!(f(1, 2)(3)(4)(5), 15);
assert_eq!(f(1)(2)(3)(4)(5), 15);
```

```rust
use curry::*;

let f = Uncurry(|a| move |b| move |c| move |d| move |e| a + b + c + d + e);
assert_eq!(f(1, 2, 3, 4, 5), 15);
assert_eq!(f(1, 2, 3, 4)(5), 15);
assert_eq!(f(1, 2, 3)(4)(5), 15);
assert_eq!(f(1, 2)(3)(4)(5), 15);
assert_eq!(f(1)(2)(3)(4)(5), 15);
```
