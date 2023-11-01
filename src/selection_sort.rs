fn selection_sort<T: Ord + Copy>(a: &mut [T]) {
    todo!()
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
