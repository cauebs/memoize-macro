# memoize-macro
Attribute macro for memoizing functions using a map.

## Examples
```rust
use memoize_macro::memoize;
use std::collections::{BTreeMap, HashMap};

#[memoize] // uses hashmap by default
fn fib_default(n: u64) -> u64 {
    match n {
        0 | 1 => 1,
        n => fib_default(n - 2) + fib_default(n - 1),
    }
}

#[memoize(HashMap)]
fn fib_hash(n: u64) -> u64 {
    match n {
        0 | 1 => 1,
        n => fib_hash(n - 2) + fib_hash(n - 1),
    }
}

#[memoize(BTreeMap)]
fn fib_btree(n: u64) -> u64 {
    match n {
        0 | 1 => 1,
        n => fib_btree(n - 2) + fib_btree(n - 1),
    }
}
```

Both the function's parameters' types and the return type must be `Clone`.
Each choice of map might impose additional requirements (e.g. `Hash + Eq` for `HashMap` and `Ord` for `BTreeMap`).

You can use other data structures, as long as they offer `get` and `insert` methods with the same signature as the ones in the `std` maps.
