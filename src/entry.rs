use std::fmt::Debug;
use crate::{IntMap, IntMapKey};

/// A view into a single entry in a [`IntMap`], which may either be vacant or occupied.
///
/// The entry can be constructed by calling [`IntMap::entry`] with a key. It allows inspection
/// and in-place manipulation of its value without repeated lookups.
pub enum Entry<'a, K: 'a + Copy + Debug + PartialEq + IntMapKey, V: 'a> {
    /// The entry is occupied.
    Occupied(OccupiedEntry<'a, K, V>),
    /// The entry is vacant.
    Vacant(VacantEntry<'a, K, V>),
}

impl<'a, K: Copy + Debug + PartialEq + IntMapKey, V> Entry<'a, K, V> {
    #[inline]
    pub(crate) fn new(key: K, int_map: &'a mut IntMap<K, V>) -> Self {
        let (cache_ix, val_ix) = Self::indices(key, &int_map);

        match val_ix {
            Some(vals_ix) => Entry::Occupied(OccupiedEntry {
                vals_ix,
                vals: &mut int_map.cache[cache_ix],
                count: &mut int_map.count,
            }),
            None => Entry::Vacant(VacantEntry {
                key,
                int_map: int_map,
            }),
        }
    }

    fn indices(key: K, int_map: &IntMap<K, V>) -> (usize, Option<usize>) {
        let cache_ix = int_map.calc_index(&key);

        let vals = &int_map.cache[cache_ix];

        for (vals_ix, (k, _)) in vals.iter().enumerate() {
            if k == &key {
                return (cache_ix, Some(vals_ix));
            }
        }

        (cache_ix, None)
    }
}

/// A view into an occupied entry in a [`IntMap`]. It is part of the [`Entry`] enum.
pub struct OccupiedEntry<'a, K: 'a + Copy + Debug + PartialEq + IntMapKey, V: 'a> {
    // Index to vals, guaranteed to be valid
    vals_ix: usize,
    // Element of IntMap::cache, guaranteed to be non-empty
    vals: &'a mut Vec<(K, V)>,
    // IntMap::count, guaranteed to be non-zero
    count: &'a mut usize,
}

impl<'a, K: Copy + Debug + PartialEq + IntMapKey, V> OccupiedEntry<'a, K, V> {
    /// Gets a reference to the value in the entry.
    pub fn get(&self) -> &V {
        // Safety: We didn't modify the cache since we calculated the index
        &self.vals.get(self.vals_ix).unwrap().1
    }

    /// Gets a mutable reference to the value in the entry.
    pub fn get_mut(&mut self) -> &mut V {
        // Safety: We didn't modify the cache since we calculated the index
        &mut self.vals.get_mut(self.vals_ix).unwrap().1
    }

    /// Converts the entry into a mutable reference to the value in the entry with a
    /// lifetime bound to the [`IntMap`] itself.
    pub fn into_mut(self) -> &'a mut V {
        // Safety: We didn't modify the cache since we calculated the index
        &mut self.vals.get_mut(self.vals_ix).unwrap().1
    }

    /// Sets the value of the entry and returns the old value.
    pub fn insert(&mut self, value: V) -> V {
        std::mem::replace(&mut self.vals[self.vals_ix].1, value)
    }

    /// Removes the value out of the entry and returns it.
    pub fn remove(self) -> V {
        // Warning: We modify the cache here, so the index is now invalid
        *self.count -= 1;
        let kv = self.vals.swap_remove(self.vals_ix);

        kv.1
    }
}

/// A view into a vacant entry in a [`IntMap`]. It is part of the [`Entry`] enum.
pub struct VacantEntry<'a, K: 'a + Copy + Debug + PartialEq + IntMapKey, V: 'a> {
    key: K,
    int_map: &'a mut IntMap<K, V>,
}

impl<'a, K: 'a + Copy + Debug + PartialEq + IntMapKey, V: 'a> VacantEntry<'a, K, V> {
    pub fn insert(self, value: V) -> &'a mut V {
        self.int_map.insert(self.key, value);
        return self.int_map.get_mut(&self.key).unwrap();
    }
}
