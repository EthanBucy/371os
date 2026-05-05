#![no_main]
#![no_std]
#![feature(custom_test_frameworks)]
#![feature(abi_x86_interrupt)]
#![test_runner(_test_runner)]

pub mod interrupts;
pub mod serial;
pub mod vga;

pub const QEMU_PASS: u32 = 0xA;
pub const QEMU_FAIL: u32 = 0xF;

pub fn init() {
    init_sse();
    interrupts::init_idt();
}

fn init_sse() {
    use x86_64::registers::control::{Cr0, Cr0Flags, Cr4, Cr4Flags};

    unsafe {
        Cr0::update(|cr0| {
            cr0.remove(Cr0Flags::EMULATE_COPROCESSOR);
            cr0.insert(Cr0Flags::MONITOR_COPROCESSOR);
        });
        Cr4::update(|cr4| {
            cr4.insert(Cr4Flags::OSFXSR | Cr4Flags::OSXMMEXCPT_ENABLE);
        });
    }
}

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
