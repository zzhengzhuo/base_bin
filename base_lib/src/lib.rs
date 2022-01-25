#![no_std]
#![feature(allocator_api)]
#![feature(alloc_error_handler)]

#[no_mangle]
pub extern "C" fn echo(input: u32) -> u32 {
    input
}

#[alloc_error_handler]
fn alloc_panic(_: core::alloc::Layout) -> ! {
    panic!("DO NOTHING alloc_error_handler set by kernel");
}

#[panic_handler]
fn unexpected_panic(_info: &core::panic::PanicInfo) -> ! {
    loop {}
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
