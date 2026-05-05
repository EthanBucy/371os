#![no_main]
#![no_std]
#![feature(custom_test_frameworks)]
#![test_runner(_test_runner)]

pub mod serial;
pub mod vga;

pub const QEMU_PASS: u32 = 0xA;
pub const QEMU_FAIL: u32 = 0xF;

pub fn qemu_quit(code: u32) {
    unsafe {
        x86_64::instructions::port::Port::new(0xf4).write(code);
    }
}

pub fn _test_panic(info: &core::panic::PanicInfo) -> ! {
    serial_println!("[Fail]");
    serial_println!("{}", info);
    qemu_quit(QEMU_FAIL);
    loop {}
}

pub fn _test_runner(tests: &[&dyn Fn()]) {
    for (i, test) in tests.iter().enumerate() {
        serial_print!("Initiating test 0x{:02x}... ", i);
        test();
        serial_println!("[Pass]");
    }

    qemu_quit(QEMU_PASS);
}

#[cfg(test)]
#[unsafe(no_mangle)]
pub extern "C" fn _start() -> ! {
    _test_runner(&[]);
    loop {}
}

#[cfg(test)]
#[panic_handler]
fn panic(info: &core::panic::PanicInfo) -> ! {
    _test_panic(info)
}
