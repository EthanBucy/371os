use std::mem::{self, MaybeUninit};
use std::ptr;

pub const SIZE: usize = 0x80;

static mut BUS: [u8; SIZE] = [0u8; SIZE];

fn mask_bytes() -> usize {
    SIZE / 8
}

fn bus_ptr() -> *mut u8 {
    ptr::addr_of_mut!(BUS).cast::<u8>()
}

fn bit_is_set(index: usize) -> bool {
    assert!(index < SIZE, "bit index out of bounds");
    let byte_index = index / 8;
    let bit_offset = index % 8;

    // SAFETY: `byte_index` is within BUS metadata range and the raw pointer
    // is derived from BUS.
    unsafe {
        let byte = ptr::read(bus_ptr().add(byte_index));
        (byte & (1u8 << bit_offset)) != 0
    }
}

fn set_bit(index: usize) {
    assert!(index < SIZE, "bit index out of bounds");
    let byte_index = index / 8;
    let bit_offset = index % 8;

    // SAFETY: `byte_index` is in-bounds of BUS and points to a metadata byte.
    unsafe {
        let p = bus_ptr().add(byte_index);
        let byte = ptr::read(p);
        ptr::write(p, byte | (1u8 << bit_offset));
    }
}

fn init() {
    assert!(SIZE.is_power_of_two(), "SIZE must be a power of 2");

    let meta = mask_bytes();
    for idx in 0..meta {
        set_bit(idx);
    }
}

fn range_is_free(start: usize, len: usize) -> bool {
    (start..start + len).all(|i| !bit_is_set(i))
}

fn range_is_allocated(start: usize, len: usize) -> bool {
    (start..start + len).all(bit_is_set)
}

fn mark_allocated(start: usize, len: usize) {
    for i in start..start + len {
        set_bit(i);
    }
}

pub fn malloc(s: usize) -> Option<usize> {
    if s == 0 || s > SIZE {
        return None;
    }

    init();

    let meta = mask_bytes();
    if s > SIZE - meta {
        return None;
    }

    for start in meta..=SIZE - s {
        if range_is_free(start, s) {
            mark_allocated(start, s);

            // SAFETY: `start..start+s` has been checked to be in-bounds.
            unsafe {
                ptr::write_bytes(bus_ptr().add(start), 0, s);
            }

            return Some(start);
        }
    }

    None
}

pub fn setter<T>(val: T, loc: usize) {
    let size = mem::size_of::<T>();
    let end = loc
        .checked_add(size)
        .expect("setter location overflowed usize");
    assert!(end <= SIZE, "setter out of bounds");
    assert!(range_is_allocated(loc, size), "setter target not allocated");

    // SAFETY: destination range is bounds-checked and allocation-checked,
    // source points to `val`, and ranges do not overlap.
    unsafe {
        let src = ptr::addr_of!(val).cast::<u8>();
        let dst = bus_ptr().add(loc);
        ptr::copy_nonoverlapping(src, dst, size);
    }

    mem::forget(val);
}

pub fn getter<T>(loc: usize) -> T {
    let size = mem::size_of::<T>();
    let end = loc
        .checked_add(size)
        .expect("getter location overflowed usize");
    assert!(end <= SIZE, "getter out of bounds");
    assert!(range_is_allocated(loc, size), "getter source not allocated");

    let mut out = MaybeUninit::<T>::uninit();

    // SAFETY: source range is bounds-checked and allocation-checked,
    // destination is valid uninitialized storage for `T`.
    unsafe {
        let src = bus_ptr().add(loc);
        let dst = out.as_mut_ptr().cast::<u8>();
        ptr::copy_nonoverlapping(src, dst, size);
        out.assume_init()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::{Mutex, MutexGuard};

    static TEST_LOCK: Mutex<()> = Mutex::new(());

    #[derive(Copy, Clone, Debug, PartialEq, Eq)]
    struct Pair {
        a: u16,
        b: u16,
    }

    fn lock_tests() -> MutexGuard<'static, ()> {
        match TEST_LOCK.lock() {
            Ok(guard) => guard,
            Err(poisoned) => poisoned.into_inner(),
        }
    }

    fn reset_for_test() {
        // SAFETY: tests run in one process here; this resets BUS state before
        // each test scenario.
        unsafe {
            BUS = [0u8; SIZE];
        }
    }

    #[test]
    fn malloc_16_returns_some() {
        let _guard = lock_tests();
        reset_for_test();
        assert!(malloc(16).is_some());
    }

    #[test]
    fn malloc_2048_returns_none() {
        let _guard = lock_tests();
        reset_for_test();
        assert!(malloc(2048).is_none());
    }

    #[test]
    fn setter_getter_i32_round_trip() {
        let _guard = lock_tests();
        reset_for_test();
        let p = malloc(mem::size_of::<i32>()).expect("expected allocation");
        let v = 0x11223344_i32;
        setter(v, p);
        let got: i32 = getter(p);
        assert_eq!(v, got);
    }

    #[test]
    fn setter_getter_u64_round_trip() {
        let _guard = lock_tests();
        reset_for_test();
        let p = malloc(mem::size_of::<u64>()).expect("expected allocation");
        let v = 0x0123_4567_89ab_cdef_u64;
        setter(v, p);
        let got: u64 = getter(p);
        assert_eq!(v, got);
    }

    #[test]
    fn setter_getter_copy_struct_round_trip() {
        let _guard = lock_tests();
        reset_for_test();
        let p = malloc(mem::size_of::<Pair>()).expect("expected allocation");
        let v = Pair { a: 7, b: 11 };
        setter(v, p);
        let got: Pair = getter(p);
        assert_eq!(v, got);
    }

    #[test]
    fn multiple_allocations_do_not_overlap() {
        let _guard = lock_tests();
        reset_for_test();
        let p0 = malloc(16).expect("expected allocation");
        let p1 = malloc(32).expect("expected allocation");
        assert!(p0 + 16 <= p1 || p1 + 32 <= p0);
    }

    #[test]
    fn exhausting_memory_returns_none() {
        let _guard = lock_tests();
        reset_for_test();
        let usable = SIZE - mask_bytes();
        assert!(malloc(usable).is_some());
        assert!(malloc(1).is_none());
    }

    #[test]
    #[should_panic]
    fn getter_unallocated_panics() {
        let _guard = lock_tests();
        reset_for_test();
        let _v: i32 = getter(mask_bytes());
    }
}
