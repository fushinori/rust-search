use std::cmp::PartialEq;

pub fn linear_search<T: PartialEq>(list: &[T], target: &T) -> Option<usize> {
    for (i, v) in list.iter().enumerate() {
        if v == target {
            return Some(i);
        };
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
        let list = [1, 2, 3, 4, 5];
        let target = 4;
        let index = linear_search(&list, &target).unwrap();
        assert_eq!(index, 3);
    }

    #[test]
    fn vec_test() {
        let list = vec![1, 2, 3, 4, 5];
        let target = 4;
        let index = linear_search(&list, &target).unwrap();
        assert_eq!(index, 3);
    }

    #[test]
    fn return_none() {
        let list = vec![1, 2, 3, 4];
        let target = 69;
        let index = linear_search(&list, &target);
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
        b.iter(|| linear_search(&list, &target));
    }
}
