
use std::{borrow::Borrow, collections::{hash_map::DefaultHasher, HashSet}, hash::{Hash, Hasher}, mem};

fn modulo(a: u64, b: u64) -> u64 {
    (a % b + b) % b
}

/**
 * Goals for the fixed-size HashSet:
 * - All tests have to pass
 * - As few allocations as possible
 * - As little memory footprint as possible
 * - As little unsafe code as possible. It is doable without any unsafe code
 * - insert and get have to be O(1)
 *
 * Advanced thing would be to make this datastructure generic over the storage!
 */

#[derive(Debug, PartialEq, Eq)]
pub enum InsertResult<T: Eq + Hash> {
    Success,
    Duplicate(T),
    InsufficientCapacity(T),
}

/// A fixed-size HashSet. It can store a fixed number of elements. The number of elements is given
/// upon initialization and all memory should be allocated once. Otherwise, it should behave exactly
/// as std::collections::HashSet

pub struct FixedSizeHashSet<T: Eq + Hash> {
    size: usize,
    data: Vec<Option<T>>,
    count: usize,
    // Add other members as needed, but beware the memory footprint!
}

impl<T: Eq + Hash> FixedSizeHashSet<T> {
    /// Creates a new `FixedSizeHashSet` which can store `capacity` elements
    pub fn new(capacity: usize) -> FixedSizeHashSet<T> {
        let bar = std::iter::repeat_with(|| Option::<T>::None)
            .take(capacity)
            .collect::<Vec<_>>();

        Self {
            size: capacity,
            data: bar,
            count: 0,
        }
    }

    pub fn HashFunc(val: &T) -> u64 {
        let mut Hasher = DefaultHasher::new();
        val.hash(&mut Hasher);
        let hash = Hasher.finish();
        hash
    }

    /// Tries to insert the given value into this `FixedSizeHashSet`. There are three possible
    /// outcomes:
    /// 1) The operation succeeds, in which case `InsertResult::Success` is returned
    /// 2) The operation succeeds, but there was already an equal value in the HashSet. In this case, the value
    ///    is replaced and the old value is returned as a `InsertResult::Duplicate(oldValue : T)`
    /// 3) There is no more room in this HashSet, in which case `InsertResult::InsufficientCapacity(val: T)` is returned, which
    ///    contains the value
    ///
    pub fn insert(&mut self, val: T) -> InsertResult<T> {
        let mut Hash = FixedSizeHashSet::<T>::HashFunc(&val);
        //println!("{}", Hash);
        Hash = modulo(Hash, self.size as u64); // Selfmade -.-
        let mut i: usize = 0;

        while i < self.capacity() {
            let arr = &mut self.data;
            let option = arr.get((Hash as usize) + i);
            let unwraped_element = option.unwrap();
            //let mut arr_iter = arr.iter();
            match unwraped_element {
                Some(x) => {
                    // At the given Position is an Element
                    if x.eq(&val) {
                        // Is the same, return Duplicate
                        let mut swap = Some(val);
                        let mut position = modulo(Hash + i as u64, self.size as u64);
                        mem::swap(&mut swap, &mut arr[position as usize]);
                        return InsertResult::Duplicate(swap.unwrap());
                    } else {
                        // Is not the same, Iterate and insert, or return insufficient
                        i = i + 1;
                        continue;
                    }
                }
                None => {
                    // Insert NEW Element
                    arr[Hash as usize + i] = Some(val);
                    self.count += 1;
                    return InsertResult::Success;
                }
            }
        }
        return InsertResult::InsufficientCapacity(val);
    }

    /// Tries to find the given `value` inside this `FixedSizeHashSet`. If it exists, a borrow to the value is returned,
    /// otherwise `None` is returned
    pub fn get(&self, value: &T) -> Option<&T> {
        let mut Hash = FixedSizeHashSet::<T>::HashFunc(&value);
        //println!("{}", Hash);
        Hash = modulo(Hash, self.size as u64); // Selfmade -.-
        let mut i: usize = 0;
        while i < self.capacity() {
            let option = self.data.get((Hash as usize) + i).unwrap();
            //let mut arr_iter = arr.iter();
            match option {
                Some(x) => {
                    // At the given Position is an Element
                    if x.eq(&value) {
                        return Some(self.data[Hash as usize + i].as_ref().unwrap().borrow());
                    } else {
                        // Is not the same, Iterate and insert, or return insufficient
                        i = i + 1;
                        continue;
                    }
                }
                None => {
                    // None - also return NONE
                    return None;
                }
            }
        }
        return None;
    }

    /// Returns the number of elements in this `FixedSizeHashSet`
    pub fn len(&self) -> usize {
        self.count
    }

    /// Returns the capacity of this `FixedSizeHashSet`, i.e. the maximum number of elements that it can store
    pub fn capacity(&self) -> usize {
        self.size
    }

    pub fn capacity_u64(&self) -> u64 {
        let x: u64 = self.size as u64;
        x
    }

    /// Returns `true` if this `FixedSizeHashSet` contains the given value
    pub fn contains(&self, val: &T) -> bool {
        let mut Hash = FixedSizeHashSet::<T>::HashFunc(&val);
        Hash = modulo(Hash, self.capacity_u64());
        let mut i: usize = 0;


        while i < self.capacity() {
            let option = self.data.get(Hash as usize + i).unwrap();
            match option {
                Some(x) => {
                    if x.eq(val) {
                        return true;
                    } else {
                        i = i + 1;
                        continue;
                    }
                }
                None => {
                    return false;
                }
            }
            
            
        }
        return false;
    }


    /// Clears the contents of this `FixedSizeHashSet`
    pub fn clear(&mut self) {
        let bar = std::iter::repeat_with(|| Option::<T>::None)
            .take(self.size)
            .collect::<Vec<_>>();
        self.count = 0;
        self.data = bar;
    }

    /// Computes the difference between this `FixedSizeHashSet` and `other`. The difference is equal to all
    /// elements that are in `self` but not in `other`
    pub fn difference(&self, other: &Self) -> HashSet<&T> {
        let mut blub = HashSet::new();
        for element in &self.data{
            let mut b : bool = true;
            match element {
                Some(x) => {
                    for other_element in &other.data{
                        match other_element{
                            Some(other_x) => {
                                if other_x.eq(x) {
                                    b = false;
                                }
                            }
                            None =>{
                                continue;
                            }
                        }

                    }
                    if b {
                        blub.insert(element.as_ref().unwrap().borrow());
                    }
                    
                }
                None => {
                    continue;
                }
            }

        }
        return blub;
    }

    /// Computes the intersection of this `FixedSizeHashSet` and `other`. The intersection is equal to all
    /// elements that are in `self` and also in `other`
    pub fn intersection(&self, other: &Self) -> HashSet<&T> {
        let mut blub = HashSet::new();
        for element in &self.data{
            let mut b : bool = false;
            match element {
                Some(x) => {
                    for other_element in &other.data{
                        match other_element{
                            Some(other_x) => {
                                if other_x.eq(x) {
                                    b = true;
                                }
                            }
                            None =>{
                                continue;
                            }
                        }

                    }
                    if b {
                        blub.insert(element.as_ref().unwrap().borrow());
                    }
                    
                }
                None => {
                    continue;
                }
            }

        }
        return blub;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_hashset_is_empty() {
        let capacity = 4;
        let hashset = FixedSizeHashSet::<i32>::new(capacity);
        assert_eq!(0, hashset.len());
        assert_eq!(capacity, hashset.capacity());
    }

    #[test]
    fn hashset_insert() {
        let capacity = 4;
        let mut hashset = FixedSizeHashSet::<i32>::new(capacity);
        assert_eq!(InsertResult::<i32>::Success, hashset.insert(42));

        assert_eq!(1, hashset.len());
        assert_eq!(capacity, hashset.capacity());
        assert!(hashset.contains(&42));
        assert_eq!(Some(&42), hashset.get(&42));
    }

    #[test]
    fn hashset_multi_insert() {
        let capacity = 4;
        let mut hashset = FixedSizeHashSet::<i32>::new(capacity);
        for val in 0..capacity {
            assert_eq!(InsertResult::<i32>::Success, hashset.insert(val as i32));
        }

        assert_eq!(4, hashset.len());
        assert_eq!(capacity, hashset.capacity());

        for val in 0..capacity as i32 {
            assert!(hashset.contains(&val));
            assert_eq!(Some(&val), hashset.get(&val));
        }
    }

    #[test]
    fn hashset_insert_duplicate() {
        let capacity = 4;
        let mut hashset = FixedSizeHashSet::<i32>::new(capacity);
        assert_eq!(InsertResult::<i32>::Success, hashset.insert(42));
        assert_eq!(InsertResult::<i32>::Duplicate(42), hashset.insert(42));

        assert_eq!(1, hashset.len());
        assert_eq!(capacity, hashset.capacity());
        assert!(hashset.contains(&42));
        assert_eq!(Some(&42), hashset.get(&42));
    }

    #[test]
    fn hashset_insert_no_capacity() {
        let capacity = 1;
        let mut hashset = FixedSizeHashSet::<i32>::new(capacity);
        assert_eq!(InsertResult::<i32>::Success, hashset.insert(42));
        assert_eq!(
            InsertResult::<i32>::InsufficientCapacity(43),
            hashset.insert(43)
        );

        assert_eq!(1, hashset.len());
        assert_eq!(capacity, hashset.capacity());
        assert!(hashset.contains(&42));
        assert_eq!(Some(&42), hashset.get(&42));

        //Even if there is insufficient capacity, overwriting an element must still work!
        assert_eq!(InsertResult::<i32>::Duplicate(42), hashset.insert(42));
    }

    #[test]
    fn hashset_clear() {
        let capacity = 4;
        let mut hashset = FixedSizeHashSet::<i32>::new(capacity);
        for val in 0..capacity {
            assert_eq!(InsertResult::<i32>::Success, hashset.insert(val as i32));
        }

        hashset.clear();

        assert_eq!(0, hashset.len());
        assert_eq!(capacity, hashset.capacity());
    }

    #[test]
    fn hashset_clear_drops_elements() {
        static mut COUNTER: usize = 0;

        #[derive(PartialEq, Eq, Hash, Debug)]
        struct DropCounter(i32);

        impl Drop for DropCounter {
            fn drop(&mut self) {
                unsafe {
                    COUNTER += 1;
                }
            }
        }

        let capacity = 4;
        let mut hashset = FixedSizeHashSet::new(capacity);
        for val in 0..capacity {
            assert_eq!(
                InsertResult::<DropCounter>::Success,
                hashset.insert(DropCounter(val as i32))
            );
        }

        // All 4 items have to be dropped upon a clear! There are no 5assumptions about the number of drops before
        // that, however zero would be optimal!
        let drops_baseline = unsafe { COUNTER };
        hashset.clear();
        let drops_after_clear = unsafe { COUNTER };
        let drops_diff = drops_after_clear - drops_baseline;
        assert_eq!(4, drops_diff);
    }

    #[test]
    fn hashset_difference() {
        let capacity = 8;
        let mut h1 = FixedSizeHashSet::new(capacity);
        let mut h2 = FixedSizeHashSet::new(capacity);

        h1.insert(2);
        h1.insert(3);
        h1.insert(5);
        h1.insert(7);
        h1.insert(11);
        h1.insert(13);

        h2.insert(3);
        h2.insert(5);
        h2.insert(13);

        let expected_difference: HashSet<&i32> = vec![&2, &7, &11].into_iter().collect();
        let difference = h1.difference(&h2);
        assert_eq!(expected_difference, difference);
    }

    #[test]
    fn hashset_intersection() {
        let capacity = 8;
        let mut h1 = FixedSizeHashSet::new(capacity);
        let mut h2 = FixedSizeHashSet::new(capacity);

        h1.insert(2);
        h1.insert(3);
        h1.insert(5);
        h1.insert(7);
        h1.insert(11);
        h1.insert(13);

        h2.insert(3);
        h2.insert(5);
        h2.insert(13);

        let expected_union: HashSet<&i32> = vec![&3, &5, &13].into_iter().collect();
        let union = h1.intersection(&h2);
        assert_eq!(expected_union, union);
    }
}


