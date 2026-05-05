#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(osirs::_test_runner)]

#[unsafe(no_mangle)]
pub extern "C" fn _start() -> ! {
    osirs::_test_runner(&[&trigger_stack_overflow]);
    osirs::qemu_quit(osirs::QEMU_FAIL);
    loop {}
}

fn trigger_stack_overflow() {
    osirs::init();
    stack_overflow();
}

#[allow(unconditional_recursion)]
fn stack_overflow() {
    stack_overflow();
    unsafe {
        core::ptr::read_volatile(&0usize);
    }
}

#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! {
    osirs::serial_println!("[Pass]");
    osirs::qemu_quit(osirs::QEMU_PASS);
    loop {}
}
