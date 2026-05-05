#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(osirs::_test_runner)]

#[unsafe(no_mangle)]
pub extern "C" fn _start() -> ! {
    osirs::_test_runner(&[&trigger_double_fault]);
    osirs::qemu_quit(osirs::QEMU_FAIL);
    loop {}
}

fn trigger_double_fault() {
    osirs::init();
    unsafe {
        core::ptr::write_volatile(0xdeadbeef as *mut u8, 42);
    }
}

#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! {
    osirs::serial_println!("[Pass]");
    osirs::qemu_quit(osirs::QEMU_PASS);
    loop {}
}
