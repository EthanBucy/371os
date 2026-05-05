static mut LATEST: usize = 0;

const MMIO: usize = 0xb8000;
pub const VGA_COLOR: u8 = 0x0f;
pub const VGA_ROWS: usize = 25;
pub const VGA_COLS: usize = 80;
pub const VGA_MAX: usize = VGA_COLS * VGA_ROWS;

fn char_to_vga(a: u8) {
    unsafe {
        let rel = (MMIO + LATEST * 2) as *mut u8;
        *rel = a;
        *rel.add(1) = VGA_COLOR;
        LATEST += 1;
    }
}

fn scroll() {
    unsafe {
        for i in VGA_COLS..VGA_MAX {
            let src = (MMIO + i * 2) as *mut u8;
            let dst = (MMIO + (i - VGA_COLS) * 2) as *mut u8;
            *dst = *src;
            *dst.add(1) = VGA_COLOR;
        }

        for i in (VGA_MAX - VGA_COLS)..VGA_MAX {
            let dst = (MMIO + i * 2) as *mut u8;
            *dst = b' ';
            *dst.add(1) = VGA_COLOR;
        }

        LATEST -= VGA_COLS;
    }
}

pub fn str_to_vga(s: &str) {
    let bytes = s.as_bytes();

    for byte in bytes {
        unsafe {
            if LATEST >= VGA_MAX {
                scroll();
            }

            match *byte {
                b'\n' => {
                    LATEST = ((LATEST / VGA_COLS) + 1) * VGA_COLS;
                    if LATEST >= VGA_MAX {
                        scroll();
                    }
                }
                byte => char_to_vga(byte),
            }
        }
    }
}

pub fn vga_ascii_at(cell: usize) -> u8 {
    assert!(cell < VGA_MAX);
    unsafe { *((MMIO + cell * 2) as *const u8) }
}

pub fn vga_color_at(cell: usize) -> u8 {
    assert!(cell < VGA_MAX);
    unsafe { *((MMIO + cell * 2 + 1) as *const u8) }
}

pub fn vga_clear() {
    unsafe {
        for i in 0..VGA_MAX {
            *((MMIO + i * 2) as *mut u8) = b' ';
            *((MMIO + i * 2 + 1) as *mut u8) = VGA_COLOR;
        }
        LATEST = 0;
    }
}

pub fn vga_reset_cursor() {
    unsafe {
        LATEST = 0;
    }
}

pub struct Dummy {}

impl core::fmt::Write for Dummy {
    fn write_str(&mut self, s: &str) -> core::fmt::Result {
        str_to_vga(s);
        Ok(())
    }
}

pub fn _print(args: core::fmt::Arguments) {
    use core::fmt::Write;
    let mut d = Dummy {};
    d.write_fmt(args).unwrap();
}

#[macro_export]
macro_rules! print {
    ($($arg:tt)*) => {
        $crate::vga::_print(format_args!($($arg)*))
    };
}

#[macro_export]
macro_rules! println {
    () => {
        $crate::print!("\n")
    };
    ($($arg:tt)*) => {
        $crate::print!("{}\n", format_args!($($arg)*))
    };
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn memcpy(dst: *mut u8, src: *const u8, n: usize) -> *mut u8 {
    for i in 0..n {
        unsafe {
            *dst.add(i) = *src.add(i);
        }
    }
    dst
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn memset(dst: *mut u8, value: i32, n: usize) -> *mut u8 {
    for i in 0..n {
        unsafe {
            *dst.add(i) = value as u8;
        }
    }
    dst
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn memcmp(a: *const u8, b: *const u8, n: usize) -> i32 {
    for i in 0..n {
        let av;
        let bv;
        unsafe {
            av = *a.add(i);
            bv = *b.add(i);
        }
        if av != bv {
            return av as i32 - bv as i32;
        }
    }
    0
}
