use std::cmp::Ordering;

pub fn find<A: AsRef<[T]>, T: Ord>(array: A, key: T) -> Option<usize> {
    let array = array.as_ref();
    let mut left = 0;
    let mut right = array.len();
    while right > left {
        let mid = (right - left) / 2 + left;
        match array[mid].cmp(&key) {
            Ordering::Equal => return Some(mid),
            Ordering::Less => {
                left = mid + 1;
            }
            Ordering::Greater => {
                right = mid;
            }
        }
    }
    None
}
