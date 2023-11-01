use std::ptr;

fn selection_sort<T: Ord + Copy>(a: &mut [T]) {
    for i in 0..a.len() {
        let mut min = i;
        for j in i..a.len() {
            if a[j] < a[min] {
                min = j;
            }
        }
        unsafe {
            ptr::swap(&mut a[i], &mut a[min]);
        }
    }
}

#[test]
fn test_selection_sort() {
    use crate::generate_random_array;
    let mut sorted = generate_random_array();
    let mut test_sort = sorted.clone();
    println!("{:?}", sorted);
    sorted.sort();
    selection_sort(&mut test_sort);
    assert_eq!(sorted, test_sort);
}
