use std::ptr;

fn quick_sort<T: Ord + Copy>(a: &mut [T]) {
    quick_sort_(a, 0, a.len() - 1);
}

fn quick_sort_<T: Ord + Copy>(a: &mut [T], lo: usize, hi: usize) {
    if lo < hi {
        let p = partition(a, lo, hi);
        quick_sort_(a, lo, p.saturating_sub(1));
        quick_sort_(a, p + 1, hi);
    }
}

fn partition<T: Ord + Copy>(a: &mut [T], lo: usize, hi: usize) -> usize {
    dbg!((lo, hi));
    let pivot = a[hi];
    let mut i = lo;
    for j in lo..hi {
        if a[j] <= pivot {
            unsafe { ptr::swap(&mut a[i], &mut a[j]) };
            i += 1;
        }
    }
    unsafe { ptr::swap(&mut a[i], &mut a[hi]) }
    dbg!(i)
}

#[test]
fn test_quick_sort() {
    use crate::generate_random_array;
    let mut sorted = generate_random_array();
    let mut test_sort = sorted.clone();
    println!("{:?}", sorted);
    sorted.sort();
    quick_sort(&mut test_sort);
    assert_eq!(sorted, test_sort);
}
