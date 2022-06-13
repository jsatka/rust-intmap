[![crates.io](https://img.shields.io/crates/v/intmap.svg)](https://crates.io/crates/intmap)

# rust-intmap
Specialized hashmap for 64-bit keys.

Might be missing some functionality but you can remove, add, get and clear for now.

Be aware that no effort is made against DoS attacks.

## Performace comparison
Example bench results on ARM M1 Mac.
```
test tests::u32_get_hashmap_fxhasher    ... bench:       1,519 ns/iter (+/- 10)
test tests::u32_get_hashmap_nohash      ... bench:       1,466 ns/iter (+/- 13)
test tests::u32_get_hashmap_std         ... bench:       8,183 ns/iter (+/- 122)
test tests::u32_get_intmap              ... bench:       1,692 ns/iter (+/- 119)
test tests::u32_insert_hashmap_fxhasher ... bench:       3,349 ns/iter (+/- 50)
test tests::u32_insert_hashmap_nohash   ... bench:       4,927 ns/iter (+/- 376)
test tests::u32_insert_hashmap_std      ... bench:      18,563 ns/iter (+/- 108)
test tests::u32_insert_intmap           ... bench:       2,988 ns/iter (+/- 16)
test tests::u64_get_hashmap_fxhasher    ... bench:       1,456 ns/iter (+/- 9)
test tests::u64_get_hashmap_nohash      ... bench:       1,288 ns/iter (+/- 5)
test tests::u64_get_hashmap_std         ... bench:      10,619 ns/iter (+/- 70)
test tests::u64_get_indexmap            ... bench:       8,599 ns/iter (+/- 89)
test tests::u64_get_intmap              ... bench:       1,516 ns/iter (+/- 7)
test tests::u64_insert_hashmap_fxhasher ... bench:       3,172 ns/iter (+/- 72)
test tests::u64_insert_hashmap_nohash   ... bench:       3,254 ns/iter (+/- 94)
test tests::u64_insert_hashmap_std      ... bench:      23,872 ns/iter (+/- 176)
test tests::u64_insert_indexmap         ... bench:      10,276 ns/iter (+/- 88)
test tests::u64_insert_intmap           ... bench:       3,054 ns/iter (+/- 30)
test tests::u64_insert_intmap_entry     ... bench:       7,412 ns/iter (+/- 230)
```

# Example usage
`IntMap` API follows the usage of Rust standard `HashMap`.

Simple example with integer key.
```rust
extern crate intmap;

use intmap::IntMap;

let mut map = IntMap::<u64, u64>::new();

for i in 0..20_000 {
    map.insert(i, format!("item: {:?}", i));
}
```

Example with custom type key.
```rust
extern crate intmap;

use intmap::{IntMap, IntMapKey};

#[derive(Clone, Copy, PartialEq, std::fmt::Debug)]
struct SomeType(u32);

impl IntMapKey for SomeType {
    fn intmap_key(&self) -> u64 {
        self.0 as u64
    }
}

let mut map = IntMap::default();
map.insert(SomeType(42), 'foo');
assert_eq!(map.get(&SomeType(42)), Some(&'foo'));
```

# What makes it so fast?
*IntMap* uses a specialized hash function for u64, which multiplies the key with the largest prime for u64. By keeping the internal cache a power 2 you can avoid the expensive modulus operator as per http://stackoverflow.com/questions/6670715/mod-of-power-2-on-bitwise-operators.
```rust
#[inline]
fn hash_u64(seed: u64) -> u64 {
    let a = 11400714819323198549u64;
    let val = a.wrapping_mul(seed);
    val
}
```
