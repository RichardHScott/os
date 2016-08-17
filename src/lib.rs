#![feature(lang_items)]
#![no_std]
#![feature(const_fn)]
#![feature(unique)]
#![feature(asm)]

extern crate rlibc;
extern crate spin;
extern crate multiboot2;

#[macro_use]
extern crate bitflags;

#[macro_use]
mod vga_buffer;

mod port;

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

    unsafe {
        setup_irq();
    }

    print!("PIC programmed");

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


const PIC_1_DEFAUT_CMD_PORT: u16 = 0x20;
const PIC_1_DEFAUT_DATA_PORT: u16 = 0x21;
const PIC_1_OFFSET : u8 = 0x20;

const PIC_2_DEFAUT_CMD_PORT: u16 = 0xa0;
const PIC_2_DEFAUT_DATA_PORT: u16 = 0xa1;
const PIC_2_OFFSET : u8 = 0x28;

// Following addresses contains information on how to program the 8259 PIC
// http://stanislavs.org/helppc/8259.html
// http://wiki.osdev.org/PIC#Programming_the_PIC_chips

bitflags! {
    flags ICW1: u8 {
        const ICW4_NEEDED = 0b00000001,
        const SINGLE_ICW1 = 0b00000010,
        const FOUR_BYTE_INTTERUPT_VECTORS = 0b00000100,
        const LEVEL_TRIGGERED_MODE =  0b00001000,
        const ICW1_USE = 0b00010000,
    }
}

bitflags! {
    flags ICW4: u8 {
        const X86_MODE = 0b00000001,
    }
}

const PIC_SLAVED_ON_IRQ_TWO : u8  = 0x4;
const PIC_SLAVE_CASCADE_ID : u8  = 0x2;

unsafe fn setup_irq() {
    //set up data ports
    use port::Port;
    let mut pic_1_data_port : Port<u8> = Port::new(PIC_1_DEFAUT_CMD_PORT);
    let mut pic_2_data_port : Port<u8> = Port::new(PIC_2_DEFAUT_DATA_PORT);

    //save masks
    let pic_1_mask = pic_1_data_port.read();
    let pic_2_mask = pic_2_data_port.read();

    //set up command ports
    let mut pic_1_cmd_port : Port<u8> = Port::new(PIC_2_DEFAUT_CMD_PORT);
    let mut pic_2_cmd_port : Port<u8> = Port::new(PIC_2_DEFAUT_CMD_PORT);

    //start init sequence in cascade 
    pic_1_cmd_port.write((ICW4_NEEDED | ICW1_USE).bits);
    pic_2_cmd_port.write((ICW4_NEEDED | ICW1_USE).bits);

    pic_1_data_port.write(PIC_1_OFFSET);
    pic_2_data_port.write(PIC_2_OFFSET);

    pic_1_data_port.write(PIC_SLAVED_ON_IRQ_TWO);
    pic_2_data_port.write(PIC_SLAVE_CASCADE_ID);

    pic_1_data_port.write(X86_MODE.bits);
    pic_2_data_port.write(X86_MODE.bits);

    //restore bitmasks
    pic_1_data_port.write(pic_1_mask);
    pic_2_data_port.write(pic_2_mask);
}
