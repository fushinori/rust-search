use std::cmp::PartialOrd;

pub fn binary_search<T: PartialOrd>(list: &[T], target: &T) -> Option<usize> {
    let mut first: usize = 0;
    let mut last = list.len() - 1;
    while first <= last {
        let midpoint = (first + last) / 2;
        let val = &list[midpoint];
        if val == target {
            return Some(midpoint);
        } else if val < target {
            first = midpoint + 1;
        } else if val > target {
            last = midpoint;
        }
    }
    None
}

#[cfg(test)]
mod tests {
    extern crate test;

    use super::*;
    use crate::{LIST_SIZE, SEED};
    use test::Bencher;

    #[test]
    fn it_works() {
        let list = [1, 2, 3, 4, 5, 6];
        let target = 5;
        let index = binary_search(&list, &target).unwrap();
        assert_eq!(index, 4);
    }
    #[test]
    fn vec_test() {
        let list = vec![1, 2, 3, 4, 5];
        let target = 4;
        let index = binary_search(&list, &target).unwrap();
        assert_eq!(index, 3);
    }

    #[test]
    fn return_none() {
        let list = vec![1, 2, 3, 4];
        let target = 69;
        let index = binary_search(&list, &target);
        assert_eq!(index, None);
    }

    #[bench]
    fn benchmark(b: &mut Bencher) {
        let list = (0..LIST_SIZE).collect::<Vec<i32>>();
        let length = list.len();
        fastrand::seed(SEED);
        let target = fastrand::usize(..length);
        let target = list[target];
        println!("Searching for {} in a list of length {}", target, length);
        b.iter(|| binary_search(&list, &target));
    }
}
