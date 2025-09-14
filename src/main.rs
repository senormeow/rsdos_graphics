#![no_std]
#![feature(alloc_error_handler)]
#![no_main]
use core::arch::asm;
use core::panic::PanicInfo;

#[unsafe(link_section = ".startup")]
#[unsafe(no_mangle)]
fn _start() -> ! {
    main();
    loop {}
}

fn pokebyte(segment: u16, offset: u16, value: u8) {
    unsafe {
        asm!(
        "mov es, {0:x}",        // Move segment to ES
        "mov di, {2:x}",        // Move offset to DI
        "mov es:[di], {1:x}",   // Use ES:DI for memory access
        in(reg) segment,
        in(reg) value as u16,
        in(reg) offset,
        out("di") _             // Inform the compiler that DI is clobbered
            );
    }
}
fn put_pixel_mem(x: u16, y: u16, color: u8) {
    let offset = 320 * y + x;
    unsafe {
        let vga_buffer = ((0xA000 * 0x10) as *mut u8).add(offset as usize);
        *vga_buffer = color;
    }
}

fn put_pixel(x: u16, y: u16, color: u8) {
    pokebyte(0xA000, 320 * y + x as u16, color);
    //put_pixel_asm(x, y, color);
}

fn put_pixel_asm(x: u16, y: u16, color: u8) {
    unsafe {
        asm!("mov ah, 0x0C",
            "int 0x10",
            in("al") color,
            in("cx") x,
            in("dx") y,
        );
    }
}

fn enter_mode13h() {
    unsafe {
        asm!(
            "mov ax, 0x13",
            "int 0x10",
            options(nomem, nostack, preserves_flags)
        );
    }
}

fn main() {
    // Your code here

    enter_mode13h();
    //put_pixel_asm(0, 0, 5);

    let mut color: u8 = 12;
    for y in 0..200 as u16 {
        for x in 0..320 as u16 {
            let color = (((x / 20) + (y / 20)) % 32) as u8;
            put_pixel(x, y, color);
        }
    }

    loop {}
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}
