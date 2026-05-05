#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(osirs::_test_runner)]
#![reexport_test_harness_main = "test_main"]

use osirs::println;
use x86_64::VirtAddr;
use x86_64::structures::paging::Page;

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
pub extern "C" fn _start(boot_info: &'static bootloader::BootInfo) -> ! {
    osirs::init();

    let offset = VirtAddr::new(boot_info.physical_memory_offset);
    let mut mapper = unsafe { osirs::memory::init(offset) };
    let mut frame_allocator =
        unsafe { osirs::memory::BootInfoFrameAllocator::init(&boot_info.memory_map) };

    let page = Page::containing_address(VirtAddr::new(0xdeadbeaf000));
    osirs::memory::create_example_mapping(page, &mut mapper, &mut frame_allocator);

    let ptr: *mut u64 = page.start_address().as_mut_ptr();
    unsafe {
        ptr.write_volatile(0x_f021_f077_f065_f04e);
    }

    println!("Mapping demo complete.");

    #[cfg(test)]
    test_main();

    osirs::halt()
}
