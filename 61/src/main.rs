#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(osirs::_test_runner)]
#![reexport_test_harness_main = "test_main"]

#[cfg(not(test))]
#[panic_handler]
fn panic(info: &core::panic::PanicInfo) -> ! {
    osirs::println!("{}", info);
    osirs::halt()
}

#[cfg(test)]
#[panic_handler]
fn panic(info: &core::panic::PanicInfo) -> ! {
    osirs::_test_panic(info)
}

#[unsafe(no_mangle)]
pub extern "C" fn _start(boot_info: &'static bootloader::BootInfo) -> ! {
    osirs::init();
    osirs::init_heap(boot_info);
    osirs::snake::init();

    #[cfg(test)]
    test_main();

    osirs::halt()
}
