#![feature(array_methods)]

struct ShuffleIterator<'a, T> {
    // TODO Fill in the members of ShuffleIterator. Does it need generic types? If so, which ones?
    data: &'a [T],
    index: usize,
    seed: u32,
}

impl<'a, T> ShuffleIterator<'a, T> {
    pub fn new(data: &'a[T]) -> Self {
        Self { 
            index : 0,
            seed : rand::random(),
            data,
        }
    }



    /// Permutes the index `i` within a range [0;length) using the given `seed` value. This function is taken from
    /// the paper 'Correlated Multi-Jittered Sampling' by Andrew Kensler (Pixar Technical Memo 13-01, 2013)
    pub fn permute(mut i: u32, length: u32, seed: u32) -> u32 {
        let mut w = length - 1;
        w |= w >> 1;
        w |= w >> 2;
        w |= w >> 4;
        w |= w >> 8;
        w |= w >> 16;

        loop {
            i ^= seed;
            i = i.wrapping_mul(0xe170893d);
            //i *= 0xe170893d;
            i ^= seed >> 16;
            i ^= (i & w) >> 4;
            i ^= seed >> 8;
            i = i.wrapping_mul(0x0929eb3f);
            //i *= 0x0929eb3f;
            i ^= seed >> 23;
            i ^= (i & w) >> 1;
            i = i.wrapping_mul(1 | seed >> 27);
            //i *= 1 | seed >> 27;
            i = i.wrapping_mul(0x6935fa69);
            //i *= 0x6935fa69;
            i ^= (i & w) >> 11;
            i = i.wrapping_mul(0x74dcb303);
            //i *= 0x74dcb303;
            i ^= (i & w) >> 2;
            i = i.wrapping_mul(0x9e501cc3);
            //i *= 0x9e501cc3;
            i ^= (i & w) >> 2;
            i = i.wrapping_mul(0xc860a3df);
            //i *= 0xc860a3df;
            i &= w;
            i ^= i >> 5;
            if i < length {
                break;
            }
        }

        (i + seed) % length
    }

}

// TODO Implement Iterator for ShuffleIterator
impl <'a, I> Iterator for ShuffleIterator<'a, I>{
    type Item = &'a I;
    fn next(&mut self) -> Option<Self::Item> {
        if self.data.len() == self.index as usize {
            None
        }
        else{
            let i : u32 = self.index as u32;
            let len : u32 = self.data.len() as u32;
            let seed : u32 = self.seed;
            self.index = self.index + 1; 
            let permuted_index: usize = Self::permute(i, len, seed) as usize;
            Some(&self.data[permuted_index])
        }
    }
}

trait ShuffleExt{
    type Item;
    
    
    fn shuffle(&self) -> ShuffleIterator<Self::Item>;
}

impl<'a, T> ShuffleExt for &'a[T]
{

    fn shuffle(&self) -> ShuffleIterator<T> {
        ShuffleIterator::new(self)
    }

    type Item = T;
}


// TODO For which type do you have to implement the ShuffleExt trait?

fn main() {
    
    let data = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
    println!("{:#?}", data.as_slice());
    println!("{:#?}", data.as_slice().shuffle().collect::<Vec<_>>());
    data.as_slice().shuffle().next();
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashSet;

    #[test]
    fn shuffle_is_exhaustive() {
        let data = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
        let expected = data.iter().copied().collect::<HashSet<_>>();
        let shuffled = data.as_slice().shuffle().copied().collect::<HashSet<_>>();
        assert_eq!(expected, shuffled);
    }

    #[test]
    fn shuffle_is_random() {
        let first_100_numbers = (0..100).collect::<Vec<_>>();
        // Shuffle twice, the shuffles must not be equal. Since there are 2^32 possible seed values, the chance for two shuffles to be
        // equal is 1/2^32. This is not a lot, but can happen, so we shuffle three times and require that at least one shuffle is unique,
        // the chance for that is 1/2^64
        let s1 = first_100_numbers
            .as_slice()
            .shuffle()
            .copied()
            .collect::<Vec<_>>();
        let s2 = first_100_numbers
            .as_slice()
            .shuffle()
            .copied()
            .collect::<Vec<_>>();
        let s3 = first_100_numbers
            .as_slice()
            .shuffle()
            .copied()
            .collect::<Vec<_>>();

        let s1_unique = s1 != s2 || s1 != s3;
        let s2_unique = s2 != s1 || s2 != s3;
        assert!(s1_unique || s2_unique);
    }

    #[test]
    fn shuffle_does_not_modify_slice() {
        let data = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
        data.as_slice().shuffle().for_each(|_| {});
        // The assertion is technically irrelevant since data is declared as immutable
        assert_eq!(data, [1, 2, 3, 4, 5, 6, 7, 8, 9, 10]);
    }
}



