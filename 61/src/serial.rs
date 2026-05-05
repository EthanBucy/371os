use uart_16550::SerialPort;

#[macro_export]
macro_rules! serial_print {
    ($($arg:tt)*) => {
        $crate::serial::_print(format_args!($($arg)*));
    };
}

#[macro_export]
macro_rules! serial_println {
    () => {
        $crate::serial_print!("\n");
    };
    ($fmt:expr) => {
        $crate::serial_print!(concat!($fmt, "\n"));
    };
    ($fmt:expr, $($arg:tt)*) => {
        $crate::serial_print!(concat!($fmt, "\n"), $($arg)*);
    };
}

pub fn _print(args: core::fmt::Arguments) {
    use core::fmt::Write;

    unsafe {
        let mut serial_port = SerialPort::new(0x3f8);
        serial_port.init();
        serial_port.write_fmt(args).unwrap();
    }
}
