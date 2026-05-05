#![no_std]
#![no_main]

#[unsafe(no_mangle)]
pub extern "C" fn _start() -> ! {
    let vga = 0xb8000 as *mut u8;
    let ints: [i32; 3] = [1819043144, 1870078063, 560229490];
    let ptr = ints.as_ptr() as *const u8;

    for i in 0..12 {
        unsafe {
            *vga.add(i * 2) = *ptr.add(i);
            *vga.add(i * 2 + 1) = 0x0f;
        }
    }

    loop {}
}

#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! {
    loop {}
}
