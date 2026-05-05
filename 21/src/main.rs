use std::io::Write;

fn main() {
    let nums: [i32; 4] = [0x6c6c6548, 0x6f77206f, 0x21646c72, 0x0000000a];

    // SAFETY: `nums` is a valid in-scope value. This reinterprets the same
    // bytes through a differently typed shared reference, then only reads
    // the first 13 output bytes.
    let bytes: &[u8] = unsafe {
        let all: &[u8; 16] = std::mem::transmute(&nums);
        &all[..13]
    };

    if std::io::stdout().write_all(bytes).is_err() {
        std::process::exit(1);
    }
}
