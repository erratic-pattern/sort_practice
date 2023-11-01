use std::{mem, ptr};

fn insertion_sort<T: Ord + Copy>(a: &mut [T]) {
    for i in 1..a.len() {
        let temp = a[i];
        let mut j = i;
        while j > 0 && a[j - 1] > temp {
            a[j] = a[j - 1];
            j -= 1;
        }
        a[j] = temp;
    }
}

#[test]
fn test_insertion_sort() {
    use crate::generate_random_array;
    let mut sorted = generate_random_array();
    let mut test_sort = sorted.clone();
    println!("{:?}", sorted);
    sorted.sort();
    insertion_sort(&mut test_sort);
    assert_eq!(sorted, test_sort);
}
