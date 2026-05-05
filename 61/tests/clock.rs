#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(osirs::_test_runner)]

#[unsafe(no_mangle)]
pub extern "C" fn _start() -> ! {
    osirs::_test_runner(&[&seconds_increment, &seconds_rollover]);
    loop {}
}

fn seconds_increment() {
    osirs::clock::set_for_test(1, 2, 3);
    osirs::clock::force_second_for_test();
    assert_eq!(osirs::clock::seconds_for_test(), 4);
}

fn seconds_rollover() {
    osirs::clock::set_for_test(1, 2, 59);
    osirs::clock::force_second_for_test();
    assert_eq!(osirs::clock::seconds_for_test(), 0);
}

#[panic_handler]
fn panic(info: &core::panic::PanicInfo) -> ! {
    osirs::_test_panic(info)
}
