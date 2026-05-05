pub mod img;

const VGA: usize = 0xb8000;
const ROWS: usize = 25;
const COLS: usize = 80;

pub fn colors() {
    for color in 0..16 {
        for row in 0..ROWS {
            for col in 0..5 {
                unsafe {
                    let cell = 5 * color + COLS * row + col;
                    let color_addr = (VGA + cell * 2 + 1) as *mut u8;
                    *color_addr = (color as u8) << 4;
                }
            }
        }
    }
}

pub fn image() {
    for i in 0..(COLS * ROWS) {
        unsafe {
            let ascii_addr = (VGA + i * 2) as *mut u8;
            let color_addr = (VGA + i * 2 + 1) as *mut u8;
            *ascii_addr = b' ';
            *color_addr = img::ARR[i];
        }
    }
}
