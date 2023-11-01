fn insertion_sort<T: Ord + Copy>(a: &mut [T]) {
    todo!()
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
