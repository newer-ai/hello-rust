#[cfg(test)]
mod tests {
    use itertools::Itertools;

    #[test]
    fn test_iterator_filter() {
        let nums = 0..=5;
        let evens: Vec<_> = nums.filter(|x| *x % 2 == 0).collect();

        assert_eq!(evens, vec![0, 2, 4]);
    }

    #[test]
    fn test_iterator_filter_map() {
        let words = vec!["One", "2", "Three", "4"];
        let nums: Vec<_> = words.iter().filter_map(|x| x.parse::<i32>().ok()).collect();

        assert_eq!(nums, vec![2, 4]);
    }

    #[test]
    fn test_iterator_map() {
        let nums = [1, 2, 3];
        let squares: Vec<_> = nums.iter().map(|x| x * x).collect();

        assert_eq!(squares, vec![1, 4, 9]);
    }

    #[test]
    fn test_iterator_flat_map() {
        let words = ["Hello", "World"];
        let chars: Vec<_> = words.iter().flat_map(|w| w.chars()).collect();

        assert_eq!(
            chars,
            vec!['H', 'e', 'l', 'l', 'o', 'W', 'o', 'r', 'l', 'd']
        );
    }

    #[test]
    fn test_iterator_take_while() {
        let nums = 0..=5;
        let taken: Vec<_> = nums.take_while(|x| *x < 4).collect();

        assert_eq!(taken, vec![0, 1, 2, 3]);
    }

    #[test]
    fn test_iterator_take_while_with_disorderly_array() {
        let nums = [0, 2, 4, 1, 3, 5];
        let taken: Vec<_> = nums.iter().take_while(|x| **x < 4).collect();

        assert_eq!(taken, vec![&0, &2]);
    }

    #[test]
    fn test_iterator_step_by() {
        let nums = 0..10;
        let taken: Vec<_> = nums.step_by(2).collect();

        assert_eq!(taken, vec![0, 2, 4, 6, 8]);
    }

    #[test]
    fn test_iterator_skip_while() {
        let nums = 0..=5;
        let taken: Vec<_> = nums.skip_while(|x| *x < 4).collect();

        assert_eq!(taken, vec![4, 5]);
    }

    #[test]
    fn test_iterator_chain() {
        let a = [3, 4];
        let b = [1, 2];
        let v: Vec<_> = a.iter().chain(b.iter()).collect();

        assert_eq!(v, vec![&3, &4, &1, &2]);
    }

    #[test]
    fn test_iterator_cycle() {
        let nums = [1, 2];
        let v: Vec<_> = nums.iter().cycle().take(4).collect();

        assert_eq!(v, vec![&1, &2, &1, &2]);
    }

    #[test]
    fn test_iterator_zip() {
        let names = ["Alice", "Bob"];
        let ages = [20, 25];

        let pairs: Vec<_> = names.iter().zip(ages.iter()).collect();

        assert_eq!(pairs, vec![(&"Alice", &20), (&"Bob", &25)]);
    }

    #[test]
    fn test_iterator_enumerate() {
        let nums = [10, 20];

        let with_idx: Vec<_> = nums.iter().enumerate().collect();

        assert_eq!(with_idx, vec![(0, &10), (1, &20)]);
    }

    #[test]
    fn test_iterator_unique() {
        let nums = [10, 10, 20, 20];

        let v: Vec<_> = nums.iter().unique().collect();

        assert_eq!(v, vec![&10, &20]);
    }

    #[test]
    fn test_iterator_sorted() {
        let nums = [1, 3, 2, 5, 4];
        let sorted: Vec<_> = nums.iter().sorted().collect();

        assert_eq!(sorted, vec![&1, &2, &3, &4, &5]);
    }

    #[test]
    fn test_iterator_sorted_by() {
        let nums = [1, 3, 2, 5, 4];
        let sorted: Vec<_> = nums
            .iter()
            .sorted_by(|x, y| Ord::cmp(*x, *y).reverse())
            .collect();

        assert_eq!(sorted, vec![&5, &4, &3, &2, &1]);
    }

    #[test]
    fn test_iterator_inspect() {
        let nums = 0..4;

        let squares: Vec<_> = nums
            .inspect(|x| println!("before {}", x))
            .map(|x| x * x)
            .inspect(|x| println!("after {}", x))
            .collect();

        assert_eq!(squares, vec![0, 1, 4, 9]);
    }

    #[test]
    fn test_iterator_peekable() {
        let nums = 0..4;
        let mut iter = nums.peekable();

        assert_eq!(iter.peek(), Some(&0));
        assert_eq!(iter.next(), Some(0));
        assert_eq!(iter.next(), Some(1));
    }

    #[test]
    fn test_iterator_scan() {
        let nums = 0..4;
        let v: Vec<_> = nums
            .scan(0, |acc, x| {
                *acc += x;
                Some(*acc)
            })
            .collect();

        assert_eq!(v, vec![0, 1, 3, 6]);
    }

    #[test]
    fn test_iterator_cloned() {
        let nums = [0, 1, 2];
        let v: Vec<_> = nums.iter().cloned().map(|x| x * x).collect();

        assert_eq!(v, vec![0, 1, 4]);
    }

    #[test]
    fn test_iterator_copied() {
        let nums = [0, 1, 2];
        let v: Vec<_> = nums.iter().copied().map(|x| x * x).collect();

        assert_eq!(v, vec![0, 1, 4]);
    }

    #[test]
    fn test_iterator_rev() {
        let nums = 0..3;
        let v: Vec<_> = nums.rev().collect();

        assert_eq!(v, vec![2, 1, 0]);
    }
}
