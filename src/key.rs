/// Enables type to be used as a key in `IntMap`.
/// 
/// Essentially as simple as casting the type into `u64`. Safe implementation requires
/// that outputs do not produce colliding values.
///
/// # Example
///
/// ```
/// use intmap::{IntMap, IntMapKey};
///
/// #[derive(Clone, Copy, PartialEq, std::fmt::Debug)]
/// struct SomeType(u32);
///
/// impl IntMapKey for SomeType {
///     fn intmap_key(&self) -> u64 {
///         self.0 as u64
///     }
/// }
///
/// let mut map = IntMap::default();
///
/// map.insert(SomeType(7), 'a');
/// map.insert(SomeType(121), 'b');
/// map.insert(SomeType(982343242), 'c');
///
/// assert_eq!(Some(&'a'), map.get(&SomeType(7)));
/// assert_eq!(Some(&'b'), map.get(&SomeType(121)));
/// assert_eq!(Some(&'c'), map.get(&SomeType(982343242)));
/// ```
pub trait IntMapKey {
    fn intmap_key(&self) -> u64;
}

impl IntMapKey for u8 {
    fn intmap_key(&self) -> u64 {
        u64::from(*self)
    }
}

impl IntMapKey for u16 {
    fn intmap_key(&self) -> u64 {
        u64::from(*self)
    }
}
impl IntMapKey for u32 {
    fn intmap_key(&self) -> u64 {
        u64::from(*self)
    }
}

impl IntMapKey for u64 {
    fn intmap_key(&self) -> u64 {
        *self
    }
}

impl IntMapKey for usize {
    fn intmap_key(&self) -> u64 {
        *self as u64
    }
}

impl IntMapKey for i8 {
    fn intmap_key(&self) -> u64 {
        *self as u64
    }
}

impl IntMapKey for i16 {
    fn intmap_key(&self) -> u64 {
        *self as u64
    }
}

impl IntMapKey for i32 {
    fn intmap_key(&self) -> u64 {
        *self as u64
    }
}

impl IntMapKey for i64 {
    fn intmap_key(&self) -> u64 {
        *self as u64
    }
}

impl IntMapKey for isize {
    fn intmap_key(&self) -> u64 {
        *self as u64
    }
}