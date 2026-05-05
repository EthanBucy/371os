#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(osirs::_test_runner)]

use osirs::{println, vga};

#[unsafe(no_mangle)]
pub extern "C" fn _start() -> ! {
    osirs::_test_runner(&[
        &test_println_simple,
        &test_println_wrap,
        &test_println_scroll_newlines,
        &test_println_scroll_wrap,
    ]);

    loop {}
}

#[panic_handler]
fn panic(info: &core::panic::PanicInfo) -> ! {
    osirs::_test_panic(info)
}

fn test_println_simple() {
    vga::vga_clear();
    println!("abc");
    assert_eq!(vga::vga_ascii_at(0), b'a');
    assert_eq!(vga::vga_ascii_at(1), b'b');
    assert_eq!(vga::vga_ascii_at(2), b'c');
    assert_eq!(vga::vga_color_at(0), vga::VGA_COLOR);
}

fn test_println_wrap() {
    vga::vga_clear();
    println!("{:081x}", 1);
    assert_eq!(vga::vga_ascii_at(0), b'0');
    assert_eq!(vga::vga_ascii_at(79), b'0');
    assert_eq!(vga::vga_ascii_at(80), b'1');
    assert_eq!(vga::vga_color_at(80), vga::VGA_COLOR);
}

fn test_println_scroll_newlines() {
    vga::vga_clear();

    for i in 0..26 {
        println!("{:x}", i);
    }

    assert_eq!(vga::vga_ascii_at(0), b'2');
    assert_eq!(vga::vga_color_at(0), vga::VGA_COLOR);
}

fn test_println_scroll_wrap() {
    vga::vga_clear();

    for _ in 0..30 {
        println!("{:081x}", 1);
    }

    assert_eq!(vga::vga_color_at(0), vga::VGA_COLOR);
    assert_eq!(vga::vga_color_at(vga::VGA_COLS), vga::VGA_COLOR);
}
