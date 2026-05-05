pub fn split_at_mut(values: &mut [i32], mid: usize) -> (&mut [i32], &mut [i32]) {
    let len = values.len();
    let mid = std::cmp::min(mid, len);
    let ptr = values.as_mut_ptr();

    // SAFETY: `ptr` comes from a valid mutable slice allocation; `mid` is
    // clamped to `len`, so both slice lengths are in-bounds; `ptr.add(mid)`
    // stays within the same allocation; and the two resulting ranges
    // [0, mid) and [mid, len) do not overlap.
    unsafe {
        (
            std::slice::from_raw_parts_mut(ptr, mid),
            std::slice::from_raw_parts_mut(ptr.add(mid), len - mid),
        )
    }
}

#[cfg(test)]
mod tests {
    use super::split_at_mut;

    #[test]
    fn split_in_middle() {
        let mut values = vec![1, 2, 3, 4, 5, 6];
        let (left, right) = split_at_mut(&mut values, 3);
        assert_eq!(left, &[1, 2, 3]);
        assert_eq!(right, &[4, 5, 6]);
    }

    #[test]
    fn split_at_zero() {
        let mut values = vec![1, 2, 3, 4];
        let (left, right) = split_at_mut(&mut values, 0);
        assert_eq!(left, &[]);
        assert_eq!(right, &[1, 2, 3, 4]);
    }

    #[test]
    fn split_at_len() {
        let mut values = vec![1, 2, 3, 4];
        let len = values.len();
        let (left, right) = split_at_mut(&mut values, len);
        assert_eq!(left, &[1, 2, 3, 4]);
        assert_eq!(right, &[]);
    }

    #[test]
    fn split_past_len_clamps() {
        let mut values = vec![1, 2, 3, 4];
        let (left, right) = split_at_mut(&mut values, 100);
        assert_eq!(left, &[1, 2, 3, 4]);
        assert_eq!(right, &[]);
    }

    #[test]
    fn mutating_both_slices_updates_original() {
        let mut values = vec![1, 2, 3, 4, 5, 6];
        let (left, right) = split_at_mut(&mut values, 3);
        left[0] = 10;
        right[0] = 40;
        assert_eq!(values, vec![10, 2, 3, 40, 5, 6]);
    }
}
