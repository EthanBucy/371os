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

    let level_4_table = x86_64::registers::control::Cr3::read().0.start_address();
    println!("Level 4 page table at: {:?}", level_4_table);
    println!("Page fault handler installed.");

    #[cfg(test)]
    test_main();

    osirs::halt()
}
