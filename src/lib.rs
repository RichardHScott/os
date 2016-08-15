#![feature(lang_items)]
#![no_std]
#![feature(const_fn)]
#![feature(unique)]

extern crate rlibc;
extern crate spin;
extern crate multiboot2;

#[macro_use]
mod vga_buffer;

#[no_mangle]
pub extern fn rust_main(multiboot_information_address: usize) {
    let hello = b"Hello world!";
    let color_byte = 0x1f; //white on blue

    let mut hello_colored = [color_byte; 24];
    for (i, char_byte) in hello.into_iter().enumerate() {
        hello_colored[i*2] = *char_byte;
    }

    let buffer_ptr = (0xb8000 + 1988) as *mut _;
    unsafe {
        *buffer_ptr = hello_colored;
    }

    vga_buffer::clear_screen();
    println!("Hello world{}", "!");

    let boot_info = unsafe{ multiboot2::load(multiboot_information_address) };
    let memory_map_tag = boot_info.memory_map_tag()
        .expect("Memory map tag required");

    println!("memory areas:");
    for area in memory_map_tag.memory_areas() {
        println!("    start: 0x{:x}, length: 0x{:x}",
            area.base_addr, area.length);
    }

    let elf_sections_tag = boot_info.elf_sections_tag()
    .expect("Elf-sections tag required");

    println!("kernel sections:");
    for section in elf_sections_tag.sections() {
        println!("    addr: 0x{:x}, size: 0x{:x}, flags: 0x{:x}",
            section.addr, section.size, section.flags);
    }

    panic!("at the disco!");

    loop {}
}

#[lang = "eh_personality"] extern fn eh_personality() {}

#[lang = "panic_fmt"]
extern "C" fn panic_fmt(fmt: core::fmt::Arguments, file: &str, line: u32) -> ! {
    println!("\n\nPANIC in {} at line {}:", file, line);
    println!("    {}", fmt);
    loop{}
}

#[allow(non_snake_case)]
#[no_mangle]
pub extern "C" fn _Unwind_Resume() -> ! {
    loop {}
}