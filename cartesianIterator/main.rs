

#[derive(Clone, Debug)]
struct Cartesian<O, I, B>
where
    O: Iterator<Item=B>,
    I: Iterator + Clone,
    B: Copy,
{
    outer_iterator: O,
    inner_iterator: I,
    last_item: Option<B>,
    inner_iterator_copy: I,
}


impl<O, I, B> Cartesian<O, I, B>
where
    O: Iterator<Item=B>,
    I: Iterator + Clone,
    B: Copy,
{
    // provide a new method for Cartesian that accepts the inner and outer iterators by value
    fn new(outer_iterator: O, inner_iterator: I) -> Self {
        Self{
            outer_iterator,
            last_item: None,
            inner_iterator: inner_iterator.clone(),
            inner_iterator_copy: inner_iterator,
        }
    }
}


impl<O,I,B>Iterator for Cartesian<O, I, B>
where
    O: Iterator<Item = B>,
    I: Iterator + Clone,
    B: Copy,
{
    type Item = (I::Item,O::Item);
    fn next(&mut self) -> Option<<Self as Iterator>::Item> {

        if self.last_item.is_none(){
            self.last_item = self.outer_iterator.next(); // Check for outer_Empty edge case
            if self.last_item.is_none(){
                return None
            }
        }
        
        let mut inner_item = self.inner_iterator.next();
        if inner_item.is_none(){ // When Inner_Iterator is done
            self.inner_iterator = self.inner_iterator_copy.clone(); // Reset inner
            inner_item = self.inner_iterator.next(); // Advance outer
            if inner_item.is_none(){ // Inner iter edge case
                return None
            }

            //Advance outer
            self.last_item = self.outer_iterator.next();
            if self.last_item.is_none() {// Outer is also done
                return None
            }
        } // Inner has value, outer has value, return value
        Some((inner_item.unwrap(),self.last_item.unwrap()))
    }
}


trait CartesianIteratorExt<O, I, B>
where
    O: Iterator<Item = B>,
    I: Clone + Iterator,
    B: Copy,
{

    fn cartesian(self, other: O) -> Cartesian<O, I, B>;
    type Outer;
    type Inner;
    type OuterItem;
}

impl <O, I, B> CartesianIteratorExt<O, I, B> for I
where
    O: Iterator<Item = B>,
    I: Iterator + Clone,
    B: Copy,
{
    type Outer = O;
    type Inner = I;
    type OuterItem = B;

    fn cartesian(self, other: O) -> Cartesian<Self::Outer, Self::Inner, Self::OuterItem> {
        Cartesian::new(other, self)
    }
}

fn main() {
    // A small demonstration of how to use the cartesian iterator:
    // This first example is equivalent to a nested for-loop:
    // for y in 0..4 {
    //    for x in 0..4 {
    //        println!("{},{}", x, y);
    //    }
    // }
    for (x, y) in (0..4).cartesian(0..4) {
        println!("{},{}", x, y);
    }

    // It also works in three dimensions, which is equivalent to three nested for-loop!
    for ((x, y), z) in (0..3).cartesian(0..3).cartesian(0..3) {
        println!("{},{},{}", x, y, z);
    }

    // It also works for string slices, since these are Copy as well!
    let strs1 = ["Tir", "Nef", "Eth", "Ith", "Tal"];
    let strs2 = ["Sur", "Ber", "Jah", "Cham", "Zod"];
    for (x, y) in strs1.iter().cartesian(strs2.iter()) {
        println!("{}{}", x, y);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn cartesian_of_empty_ranges() {
        let actual = (0..0).cartesian(1..1).collect::<Vec<_>>();
        assert_eq!(Vec::<(i32, i32)>::new(), actual);
    }

    #[test]
    fn cartesian_with_one_empty_range() {
        // Test with both the inner range empty and the outer range empty
        assert_eq!(
            Vec::<(i32, i32)>::new(),
            (0..0).cartesian(1..10).collect::<Vec<_>>()
        );
        assert_eq!(
            Vec::<(i32, i32)>::new(),
            (0..10).cartesian(1..1).collect::<Vec<_>>()
        );
    }

    #[test]
    fn cartesian_2d_of_integers() {
        let actual = (0..3).cartesian(4..7).collect::<Vec<_>>();
        //Notice the order of the elements in the tuple! This defines which iterator is the outer one and which
        //the inner one!
        let expected = vec![
            (0, 4),
            (1, 4),
            (2, 4),
            (0, 5),
            (1, 5),
            (2, 5),
            (0, 6),
            (1, 6),
            (2, 6),
        ];
        assert_eq!(expected, actual);
    }

    #[test]
    fn cartesian_3d_of_integers() {
        // You should also be able to chain cartesian iterators!
        // Here, the order of inner/outer iterators becomes even more important!
        let actual = (0..2).cartesian(2..4).cartesian(4..6).collect::<Vec<_>>();
        let expected = vec![
            ((0, 2), 4),
            ((1, 2), 4),
            ((0, 3), 4),
            ((1, 3), 4),
            ((0, 2), 5),
            ((1, 2), 5),
            ((0, 3), 5),
            ((1, 3), 5),
        ];
        assert_eq!(expected, actual);
    }

    #[test]
    fn cartesian_2d_of_string_slices() {
        let str1 = ["a", "b", "c"];
        let str2 = ["1", "2"];
        let actual = str1
            .iter()
            .copied()
            .cartesian(str2.iter().copied())
            .collect::<Vec<_>>();
        let expected = vec![
            ("a", "1"),
            ("b", "1"),
            ("c", "1"),
            ("a", "2"),
            ("b", "2"),
            ("c", "2"),
        ];
        assert_eq!(expected, actual);
    }
}





