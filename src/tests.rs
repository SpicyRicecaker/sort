use rand::prelude::*;
use crate::heap::heap_sort;

#[test]
fn test_heap() {
    let mut data = vec![5, 2, 3, 4, 5];
    heap_sort(&mut data);
    assert_eq!(data, vec![2, 3, 4, 5, 5]);

    let mut data = vec![5, 6, 2, 3, 4];
    heap_sort(&mut data);
    assert_eq!(data, vec![2, 3, 4, 5, 6]);
}

#[test]
fn test_random_heap() {
    let mut rng = rand::thread_rng();
    let mut data = Vec::with_capacity(100);
    for _ in 0..10 {
        data.push(rng.gen_range(1..=100));
    }

    let mut data_2 = data.clone();
    data_2.sort();
    let mut data_3 = data.clone();
    heap_sort(&mut data_3);
    assert_eq!(data_2, data_3);
}