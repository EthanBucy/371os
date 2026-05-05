#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(osirs::_test_runner)]

#[unsafe(no_mangle)]
pub extern "C" fn _start() -> ! {
    osirs::_test_runner(&[&init_and_continue]);
    loop {}
}

fn init_and_continue() {
    osirs::init();
    x86_64::instructions::interrupts::int3();
    assert!(true);
}

#[panic_handler]
fn panic(info: &core::panic::PanicInfo) -> ! {
    osirs::_test_panic(info)
}
