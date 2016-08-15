#![no_std]

#[no_mangle]
pub unsafe extern fn memcpy(destination : *mut u8, source : *const u8, num : usize) -> usize {
//void * memcpy ( void * destination, const void * source, size_t num )
    let mut i = 0;

    while i < num {
        *destination.offset(i as isize) = *source.offset(i as isize);
        i += 1;
    }

    i
}