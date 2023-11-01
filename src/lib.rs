mod bubble_sort;
mod insertion_sort;
mod merge_sort;
mod quick_sort;
mod selection_sort;

pub fn generate_random_array() -> Vec<u32> {
    const MAX_SIZE: usize = 20;
    const MIN_SIZE: usize = 10;
    const MIN_ELEM: u32 = 0;
    const MAX_ELEM: u32 = 100;
    let size = MIN_SIZE + ((MAX_SIZE - MIN_SIZE) as f64 * rand::random::<f64>()) as usize;
    let mut rand_array = Vec::with_capacity(size);
    for _ in 0..size {
        rand_array.push(MIN_ELEM + ((MAX_ELEM - MIN_ELEM) as f64 * rand::random::<f64>()) as u32);
    }
    rand_array
}
