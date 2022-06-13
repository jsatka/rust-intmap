#![feature(test)]

extern crate intmap;
extern crate indexmap;
extern crate nohash_hasher;
extern crate rand;
extern crate rustc_hash;
extern crate test;

#[cfg(test)]
mod tests {
    use std::hash::BuildHasherDefault;
    use rand::{prelude::Distribution, distributions::Standard};
    use test::Bencher;
    use super::*;

    const VEC_COUNT: usize = 1000;

    // ********** HashMap with default hasher **********

    #[bench]
    fn u32_get_hashmap_default(b: &mut Bencher) {
        use std::collections::HashMap;
        let data = get_random_range::<u32>(VEC_COUNT);
        let mut map = HashMap::with_capacity(data.len());
        
        for s in data.iter() {
            test::black_box(map.insert(*s, *s));
        }

        b.iter(|| {
            for s in data.iter() {
                test::black_box({
                    map.contains_key(s);
                });
            }
        });
    }

    #[bench]
    fn u32_insert_hashmap_default(b: &mut Bencher) {
        use std::collections::HashMap;
        let data = get_random_range::<u32>(VEC_COUNT);
        let mut map = HashMap::with_capacity(data.len());

        b.iter(|| {
            map.clear();
            for s in data.iter() {
                test::black_box(map.insert(*s, *s));
            }
        });
    }

    #[bench]
    fn u64_get_hashmap_default(b: &mut Bencher) {
        use std::collections::HashMap;
        let data = get_random_range::<u64>(VEC_COUNT);
        let mut map = HashMap::with_capacity(data.len());
        
        for s in data.iter() {
            test::black_box(map.insert(*s, *s));
        }

        b.iter(|| {
            for s in data.iter() {
                test::black_box({
                    map.contains_key(s);
                });
            }
        });
    }

    #[bench]
    fn u64_insert_hashmap_default(b: &mut Bencher) {
        use std::collections::HashMap;
        let data = get_random_range::<u64>(VEC_COUNT);
        let mut map = HashMap::with_capacity(data.len());

        b.iter(|| {
            map.clear();
            for s in data.iter() {
                test::black_box(map.insert(*s, *s));
            }
        });
    }

    // ********** HashMap with FxHasher **********

    #[bench]
    fn u32_get_hashmap_fxhasher(b: &mut Bencher) {
        use std::collections::HashMap;
        use rustc_hash::FxHasher;
        let data = get_random_range::<u32>(VEC_COUNT);
        let mut map = HashMap::with_capacity_and_hasher(
            data.len(),
            BuildHasherDefault::<FxHasher>::default()
        );

        for s in data.iter() {
            test::black_box(map.insert(*s, *s));
        }

        b.iter(|| {
            for s in data.iter() {
                test::black_box({
                    map.contains_key(s);
                });
            }
        });
    }

    #[bench]
    fn u32_insert_hashmap_fxhasher(b: &mut Bencher) {
        use std::collections::HashMap;
        use rustc_hash::FxHasher;
        let data = get_random_range::<u32>(VEC_COUNT);
        let mut map = HashMap::with_capacity_and_hasher(
            data.len(),
            BuildHasherDefault::<FxHasher>::default()
        );

        b.iter(|| {
            map.clear();
            for s in data.iter() {
                test::black_box(map.insert(*s, *s));
            }
        });
    }

    #[bench]
    fn u64_get_hashmap_fxhasher(b: &mut Bencher) {
        use std::collections::HashMap;
        use rustc_hash::FxHasher;
        let data = get_random_range::<u64>(VEC_COUNT);
        let mut map = HashMap::with_capacity_and_hasher(
            data.len(),
            BuildHasherDefault::<FxHasher>::default()
        );

        for s in data.iter() {
            test::black_box(map.insert(*s, *s));
        }

        b.iter(|| {
            for s in data.iter() {
                test::black_box({
                    map.contains_key(s);
                });
            }
        });
    }

    #[bench]
    fn u64_insert_hashmap_fxhasher(b: &mut Bencher) {
        use std::collections::HashMap;
        use rustc_hash::FxHasher;
        let data = get_random_range::<u64>(VEC_COUNT);
        let mut map = HashMap::with_capacity_and_hasher(
            data.len(),
            BuildHasherDefault::<FxHasher>::default()
        );

        b.iter(|| {
            map.clear();
            for s in data.iter() {
                test::black_box(map.insert(*s, *s));
            }
        });
    }

    // ********** HashMap with NoHash hasher **********

    #[bench]
    fn u32_get_hashmap_nohash(b: &mut Bencher) {
        use std::collections::HashMap;
        use nohash_hasher::BuildNoHashHasher;
        let data = get_random_range::<u32>(VEC_COUNT);
        let mut map = HashMap::with_capacity_and_hasher(
            data.len(), 
            BuildNoHashHasher::<u32>::default()
        );

        for s in data.iter() {
            test::black_box(map.insert(*s, *s));
        }

        b.iter(|| {
            for s in data.iter() {
                test::black_box({
                    map.contains_key(s);
                });
            }
        });
    }

    #[bench]
    fn u32_insert_hashmap_nohash(b: &mut Bencher) {
        use std::collections::HashMap;
        use nohash_hasher::BuildNoHashHasher;
        let data = get_random_range::<u32>(VEC_COUNT);
        let mut map = HashMap::with_capacity_and_hasher(
            data.len(),
            BuildNoHashHasher::<u32>::default()
        );

        b.iter(|| {
            map.clear();
            for s in data.iter() {
                test::black_box(map.insert(*s, *s));
            }
        });
    }

    #[bench]
    fn u64_get_hashmap_nohash(b: &mut Bencher) {
        use std::collections::HashMap;
        use nohash_hasher::BuildNoHashHasher;
        let data = get_random_range::<u64>(VEC_COUNT);
        let mut map = HashMap::with_capacity_and_hasher(
            data.len(), 
            BuildNoHashHasher::<u64>::default()
        );

        for s in data.iter() {
            test::black_box(map.insert(*s, *s));
        }

        b.iter(|| {
            for s in data.iter() {
                test::black_box({
                    map.contains_key(s);
                });
            }
        });
    }

    #[bench]
    fn u64_insert_hashmap_nohash(b: &mut Bencher) {
        use std::collections::HashMap;
        use nohash_hasher::BuildNoHashHasher;
        let data = get_random_range::<u64>(VEC_COUNT);
        let mut map = HashMap::with_capacity_and_hasher(
            data.len(),
            BuildNoHashHasher::<u64>::default()
        );

        b.iter(|| {
            map.clear();
            for s in data.iter() {
                test::black_box(map.insert(*s, *s));
            }
        });
    }

    // ********** IndexMap **********

    #[bench]
    fn u64_get_indexmap(b: &mut Bencher) {
        use indexmap::IndexMap;
        let data = get_random_range::<u64>(VEC_COUNT);
        let mut map = IndexMap::with_capacity(data.len());
        for s in data.iter() {
            test::black_box(map.insert(*s, *s));
        }

        b.iter(|| {
            for s in data.iter() {
                test::black_box({
                    map.contains_key(s);
                });
            }
        });
    }

    #[bench]
    fn u64_insert_indexmap(b: &mut Bencher) {
        use indexmap::IndexMap;
        let data = get_random_range::<u64>(VEC_COUNT);
        let mut map = IndexMap::with_capacity(data.len());

        b.iter(|| {
            map.clear();

            for s in data.iter() {
                test::black_box(map.insert(*s, *s));
            }
        });
    }

    // ********** Intmap **********

    #[bench]
    fn u32_get_intmap(b: &mut Bencher) {
        use intmap::IntMap;
        let data = get_random_range::<u32>(VEC_COUNT);
        let mut map = IntMap::with_capacity(data.len());

        for s in data.iter() {
            map.insert(*s, *s);
        }

        b.iter(|| {
            for s in data.iter() {
                test::black_box(map.contains_key(s));
            }
        });
    }

    #[bench]
    fn u32_insert_intmap(b: &mut Bencher) {
        use intmap::IntMap;
        let data = get_random_range::<u32>(VEC_COUNT);
        let mut map = IntMap::with_capacity(data.len());

        b.iter(|| {
            map.clear();
            for s in data.iter() {
                test::black_box(map.insert(*s, *s));
            }
        });
    }

    #[bench]
    fn u64_get_intmap(b: &mut Bencher) {
        use intmap::IntMap;
        let data = get_random_range::<u64>(VEC_COUNT);
        let mut map = IntMap::with_capacity(data.len());

        for s in data.iter() {
            map.insert(*s, *s);
        }

        b.iter(|| {
            for s in data.iter() {
                test::black_box(map.contains_key(s));
            }
        });
    }

    #[bench]
    fn u64_insert_intmap(b: &mut Bencher) {
        use intmap::IntMap;
        let data = get_random_range::<u64>(VEC_COUNT);
        let mut map = IntMap::with_capacity(data.len());

        b.iter(|| {
            map.clear();
            for s in data.iter() {
                test::black_box(map.insert(*s, *s));
            }
        });
    }

    #[bench]
    fn u64_insert_intmap_entry(b: &mut Bencher) {
        use intmap::IntMap;
        let data = get_random_range::<u64>(VEC_COUNT);
        let mut map = IntMap::with_capacity(data.len());

        b.iter(|| {
            map.clear();
            for s in data.iter() {
                test::black_box(match map.entry(*s) {
                    intmap::Entry::Occupied(_) => panic!("unexpected while insert, i = {}", s),
                    intmap::Entry::Vacant(entry) => entry.insert(*s),
                });
            }
        });
    }

    // ********** Misc **********

    fn get_random_range<T>(count: usize) -> Vec<T> 
        where Standard: Distribution<T>, T: Ord + PartialEq
    {
        use rand::prelude::StdRng;
        use rand::{Rng, SeedableRng};

        let mut vec = Vec::new();
        let mut rng = StdRng::seed_from_u64(4242);
        for _ in 0..count {
            vec.push(rng.gen::<T>());
        }

        vec.sort();
        vec.dedup();

        vec
    }
}
