use core::marker::PhantomData;

pub unsafe fn outb(value: u8, port: u16) {
    asm!("outb %al, %dx" ::
         "{dx}"(port), "{al}"(value) ::
         "volatile");
}

pub unsafe fn inb(port: u16) -> u8 {
    let result : u8;

    asm!("inb %dx, %al" : 
         "={al}"(result) :
         "{dx}"(port) ::
         "volatile");

    result
}

pub unsafe fn outw(value: u16, port: u16) {
    asm!("outw %ax, %dx" ::
         "{dx}"(port), "{ax}"(value) ::
         "volatile");
}

pub unsafe fn inw(port: u16) -> u16 {
    let result : u16;

    asm!("inw %dx, %ax" :
         "={ax}"(result) :
         "{dx}"(port) ::
         "volatile");

    result
}

pub unsafe fn outl(value: u32, port: u16) {
    asm!("outl %eax, %dx" ::
         "{dx}"(port), "{eax}"(value) ::
         "volatile");
}

pub unsafe fn inl(port: u16) -> u32 {
    let result : u32;

    asm!("inl %dx, %eax" : 
         "={eax}"(result) :
         "{dx}"(port) ::
         "volatile");

    result
}

pub trait InOut {
    unsafe fn port_in(port : u16) -> Self;
    unsafe fn port_out(port : u16 , value: Self);
}

impl InOut for u8 {
    unsafe fn port_in(port : u16) -> u8 {
        inb(port)
    }

    unsafe fn port_out(port : u16, value : u8) {
        outb(value, port)
    }
}

impl InOut for u16 {
    unsafe fn port_in(port : u16) -> u16 {
        inw(port)
    }

    unsafe fn port_out(port : u16, value : u16) {
        outw(value, port)
    }
}

impl InOut for u32 {
    unsafe fn port_in(port : u16) -> u32 {
        inl(port)
    }

    unsafe fn port_out(port : u16, value : u32) {
        outl(value, port)
    }
}

pub struct Port<T : InOut> {
    port: u16,
    phantom : PhantomData<T>,
}

impl<T : InOut> Port<T> {
    pub unsafe fn new(port : u16) -> Port<T> {
        Port { port : port, phantom : PhantomData }
    }

    pub unsafe fn read(&mut self) -> T {
        T::port_in(self.port)
    }

     pub unsafe fn write(&mut self, value : T) {
        T::port_out(self.port, value)
    }
}