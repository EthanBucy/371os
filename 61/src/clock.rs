static mut DIGITS: [u8; 6] = [0; 6];
static mut DIGIT_LEN: usize = 0;
static mut HOURS: u8 = 0;
static mut MINUTES: u8 = 0;
static mut SECONDS: u8 = 0;
static mut TICKS: usize = 0;
static mut RUNNING: bool = false;

const TICKS_PER_SECOND: usize = 18;

pub fn reset() {
    unsafe {
        DIGITS = [0; 6];
        DIGIT_LEN = 0;
        HOURS = 0;
        MINUTES = 0;
        SECONDS = 0;
        TICKS = 0;
        RUNNING = false;
    }
}

pub fn push_digit(digit: u8) {
    unsafe {
        if RUNNING || DIGIT_LEN >= 6 {
            return;
        }

        DIGITS[DIGIT_LEN] = digit;
        DIGIT_LEN += 1;

        crate::print!("{}", (b'0' + digit) as char);

        if DIGIT_LEN == 6 {
            start_from_digits();
        }
    }
}

fn start_from_digits() {
    unsafe {
        HOURS = DIGITS[0] * 10 + DIGITS[1];
        MINUTES = DIGITS[2] * 10 + DIGITS[3];
        SECONDS = DIGITS[4] * 10 + DIGITS[5];

        if HOURS > 23 {
            HOURS %= 24;
        }
        if MINUTES > 59 {
            MINUTES %= 60;
        }
        if SECONDS > 59 {
            SECONDS %= 60;
        }

        RUNNING = true;
        TICKS = 0;
        crate::vga::vga_clear();
        draw();
    }
}

pub fn tick() {
    unsafe {
        if !RUNNING {
            return;
        }

        TICKS += 1;

        if TICKS >= TICKS_PER_SECOND {
            TICKS = 0;
            add_second();
            draw();
        }
    }
}

fn add_second() {
    unsafe {
        SECONDS += 1;

        if SECONDS >= 60 {
            SECONDS = 0;
            MINUTES += 1;
        }

        if MINUTES >= 60 {
            MINUTES = 0;
            HOURS += 1;
        }

        if HOURS >= 24 {
            HOURS = 0;
        }
    }
}

pub fn draw() {
    let (h, m, s) = unsafe { (HOURS, MINUTES, SECONDS) };
    crate::vga::vga_reset_cursor();
    crate::println!("{:02}:{:02}:{:02}", h, m, s);
}

pub fn seconds_for_test() -> u8 {
    unsafe { SECONDS }
}

pub fn set_for_test(h: u8, m: u8, s: u8) {
    unsafe {
        HOURS = h;
        MINUTES = m;
        SECONDS = s;
        TICKS = 0;
        RUNNING = true;
    }
}

pub fn force_second_for_test() {
    add_second();
}
