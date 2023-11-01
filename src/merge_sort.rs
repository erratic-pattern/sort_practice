fn merge_sort<T: Ord + Copy>(a: &mut [T]) {
    merge_sort_(a, 0, a.len() - 1);
}

fn merge_sort_<T: Ord + Copy>(a: &mut [T], lo: usize, hi: usize) {
    if lo < hi {
        let mid = lo + (hi - lo) / 2;
        merge_sort_(a, lo, mid);
        merge_sort_(a, mid + 1, hi);
        merge(a, lo, mid, hi);
    }
}

fn merge<T: Ord + Copy>(a: &mut [T], lo: usize, mid: usize, hi: usize) {
    let left: Vec<T> = Vec::from(&a[lo..=mid]);
    let right: Vec<T> = Vec::from(&a[mid + 1..=hi]);
    let (mut i, mut j, mut k) = (0, 0, lo);
    while i < left.len() && j < right.len() {
        if left[i] < right[j] {
            a[k] = left[i];
            i += 1;
        } else {
            a[k] = right[j];
            j += 1;
        }
        k += 1;
    }
    if i < left.len() {
        a[k..=hi].copy_from_slice(&left[i..]);
    }
    if j < right.len() {
        a[k..=hi].copy_from_slice(&right[j..]);
    }
}

#[test]
fn test_merge_sort() {
    use crate::generate_random_array;
    let mut sorted = generate_random_array();
    let mut test_sort = sorted.clone();
    println!("{:?}", sorted);
    sorted.sort();
    merge_sort(&mut test_sort);
    assert_eq!(sorted, test_sort);
}
