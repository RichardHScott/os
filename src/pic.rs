use port::Port;

unsafe fn start_interrupts() {
    asm!("sti" :::: "volatile");
}

struct Pic {
    data: Port<u8>,
    command: Port<u8>,
    offset: u8,
}

const CMD_END_OF_INTERRUPT :u8  = 0x20;

impl Pic {
    pub const unsafe fn new(cmd_port: u16, data_port: u16, offset: u8) -> Pic {
        Pic {
            command: Port::new(cmd_port),
            data: Port::new(data_port),
            offset: offset,
        }
    }

    pub fn handles_interrupt(&self, interrupt_id :u8) -> bool {
        //if start_id <= interrupt_d <= end_id then this PIC handles it
        //end_id = start_id + 8 as each PIC can handle 8 interrupts only
        self.offset <= interrupt_id && interrupt_id <= (self.offset + 8)
    }

    pub unsafe fn end_of_interrupt(&mut self) {
        self.command.write(CMD_END_OF_INTERRUPT);
    }
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
const PIC_SLAVED_ON_IRQ_TWO : u8  = 0x4;
const PIC_SLAVE_CASCADE_ID : u8  = 0x2;

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

pub struct ChainedPics {
    pics: [Pic; 2],
}

impl ChainedPics {
    pub const unsafe fn new(offset1: u8, offset2: u8) -> ChainedPics {
        ChainedPics {
            pics: [
                Pic::new(PIC_1_DEFAUT_CMD_PORT, PIC_1_DEFAUT_DATA_PORT, offset1),
                Pic::new(PIC_2_DEFAUT_CMD_PORT, PIC_2_DEFAUT_DATA_PORT, offset2),
            ]
        }
    }

    pub unsafe fn setup_pics(&mut self) {
        let mut wait_port: Port<u8> = Port::new(0x80); //0x80 is an unused port at this point in time
        let mut io_wait = | | { wait_port.write(0) };

        //save masks
        let pic_1_mask = self.pics[0].data.read();
        let pic_2_mask = self.pics[1].data.read();

        //start init sequence in cascade 
        self.pics[0].command.write((ICW4_NEEDED | ICW1_USE).bits);
        io_wait();
        self.pics[1].command.write((ICW4_NEEDED | ICW1_USE).bits);
        io_wait();

        self.pics[0].data.write(PIC_1_OFFSET);
        io_wait();
        self.pics[1].data.write(PIC_2_OFFSET);
        io_wait();

        self.pics[0].data.write(PIC_SLAVED_ON_IRQ_TWO);
        io_wait();
        self.pics[1].data.write(PIC_SLAVE_CASCADE_ID);
        io_wait();

        self.pics[0].data.write(X86_MODE.bits);
        io_wait();
        self.pics[1].data.write(X86_MODE.bits);
        io_wait();

        //restore bitmasks
        self.pics[0].data.write(pic_1_mask);
        io_wait();
        self.pics[1].data.write(pic_2_mask);
        io_wait();

        //start_interrupts();
    }

    pub fn handles_interrupt(&self, interrupt_id: u8) -> bool {
        self.pics.iter().any(|pic| pic.handles_interrupt(interrupt_id))
    }

    pub unsafe fn notify_end_of_interrupt(&mut self, interrupt_id: u8) {
        if self.handles_interrupt(interrupt_id) {
            //cascade handling of end of interrupt
            if self.pics[1].handles_interrupt(interrupt_id) {
                self.pics[1].end_of_interrupt();
            }
            
            self.pics[0].end_of_interrupt();
        }
    }
}

use spin::Mutex;
pub static PICS: Mutex<ChainedPics> = Mutex::new(unsafe { ChainedPics::new(PIC_1_OFFSET, PIC_2_OFFSET) } );