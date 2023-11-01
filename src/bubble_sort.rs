use std::ptr;

fn bubble_sort<T: Ord + Copy>(a: &mut [T]) {
    for i in 0..a.len() {
        for j in 0..a.len() - i - 1 {
            if a[j] > a[j + 1] {
                unsafe { ptr::swap(&mut a[j], &mut a[j + 1]) };
            }
        }
    }
}

#[test]
fn test_bubble_sort() {
    use crate::generate_random_array;
    let mut sorted = generate_random_array();
    let mut test_sort = sorted.clone();
    println!("{:?}", sorted);
    sorted.sort();
    bubble_sort(&mut test_sort);
    assert_eq!(sorted, test_sort);
}
