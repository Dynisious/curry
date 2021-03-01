
# Curry

Provides types for currying functions with up to 5 parameters.

```rust
use curry::*;

let f = Curry5(|a, b, c, d, e| a + b + c + d + e);
assert_eq!(f(1, 2, 3, 4, 5), 15);
assert_eq!(f(1, 2, 3, 4)(5), 15);
assert_eq!(f(1, 2, 3)(4)(5), 15);
assert_eq!(f(1, 2)(3)(4)(5), 15);
assert_eq!(f(1)(2)(3)(4)(5), 15);
```
