#[repr(C)]
union MyUnion {
    f: f32,
    u: u32,
}

fn add_one_with_raw_ptr(value: &mut i32) {
    let ptr = value as *mut i32;

    // SAFETY: `ptr` comes from a valid mutable reference, so it is non-null,
    // properly aligned, and points to initialized memory for `i32`.
    unsafe {
        *ptr += 1;
    }
}

fn main() {
    println!("Lab 2: Unsafe Rust demo");

    // Raw pointer example.
    let mut num = 5;
    let const_ptr: *const i32 = &num;
    let mut_ptr: *mut i32 = &mut num;

    // SAFETY: both pointers are derived from valid references to `num`,
    // and `num` is still in scope.
    unsafe {
        println!("const raw pointer value: {}", *const_ptr);
        *mut_ptr += 10;
        println!("mut raw pointer value after write: {}", *mut_ptr);
    }

    println!("num after raw pointer writes: {}", num);

    // Safe wrapper around a small unsafe operation.
    add_one_with_raw_ptr(&mut num);
    println!("num after safe wrapper call: {}", num);

    // Union example.
    let mut bits = MyUnion { f: 0.0 };

    // SAFETY: reading a union field is unsafe because the compiler cannot
    // prove which field is currently valid.
    unsafe {
        println!("union as f32 initially: {}", bits.f);
        println!("union as u32 initially: 0x{:08X}", bits.u);
    }

    bits.u = 0x3F800000;

    // SAFETY: after writing `u`, reading `f` reinterprets the same bits.
    unsafe {
        println!("union f32 after setting bits to 0x3F800000: {}", bits.f);
    }

    // Invalid pointer example (do not run):
    // let bad_ptr = 0x0usize as *const i32;
    // unsafe {
    //     println!("{}", *bad_ptr); // likely crash/undefined behavior
    // }

    // Dangling pointer example (do not run):
    // let dangling: *const i32;
    // {
    //     let temp = 123;
    //     dangling = &temp as *const i32;
    // }
    // unsafe {
    //     println!("{}", *dangling); // `temp` is out of scope here
    // }
}
