#![no_main]
#![no_std]
#![feature(custom_test_frameworks)]
#![feature(abi_x86_interrupt)]
#![feature(alloc_error_handler)]
#![test_runner(_test_runner)]

extern crate alloc;

pub mod allocator;
pub mod clock;
pub mod gdt;
pub mod interrupts;
pub mod memory;
pub mod serial;
pub mod vga;

pub const QEMU_PASS: u32 = 0xA;
pub const QEMU_FAIL: u32 = 0xF;

pub fn init() {
    gdt::init_gdt();
    init_sse();
    interrupts::init_idt();
}

pub fn init_heap(boot_info: &'static bootloader::BootInfo) {
    let offset = x86_64::VirtAddr::new(boot_info.physical_memory_offset);
    let mut mapper = unsafe { memory::init(offset) };
    let mut frame_allocator =
        unsafe { memory::BootInfoFrameAllocator::init(&boot_info.memory_map) };

    allocator::init_heap(&mut mapper, &mut frame_allocator).unwrap();
}

pub fn halt() -> ! {
    loop {
        x86_64::instructions::hlt();
    }
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
    halt()
}

pub fn test_panic_handler(info: &core::panic::PanicInfo) -> ! {
    _test_panic(info)
}

pub fn _test_runner(tests: &[&dyn Fn()]) {
    for (i, test) in tests.iter().enumerate() {
        serial_print!("Initiating test 0x{:02x}... ", i);
        test();
        serial_println!("[Pass]");
    }

    qemu_quit(QEMU_PASS);
}

#[alloc_error_handler]
fn alloc_error_handler(layout: core::alloc::Layout) -> ! {
    panic!("allocation error: {:?}", layout)
}

#[cfg(test)]
#[unsafe(no_mangle)]
pub extern "C" fn _start() -> ! {
    _test_runner(&[]);
    halt()
}

#[cfg(test)]
#[panic_handler]
fn panic(info: &core::panic::PanicInfo) -> ! {
    _test_panic(info)
}
