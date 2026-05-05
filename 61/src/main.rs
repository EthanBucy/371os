#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(osirs::_test_runner)]
#![reexport_test_harness_main = "test_main"]

use osirs::println;

#[cfg(not(test))]
#[panic_handler]
fn panic(info: &core::panic::PanicInfo) -> ! {
    println!("{}", info);
    osirs::halt()
}

#[cfg(test)]
#[panic_handler]
fn panic(info: &core::panic::PanicInfo) -> ! {
    osirs::_test_panic(info)
}

#[unsafe(no_mangle)]
pub extern "C" fn _start() -> ! {
    osirs::init();
    osirs::clock::reset();
    println!("Enter time as HHMMSS:");

    #[cfg(test)]
    test_main();

    osirs::halt()
}
