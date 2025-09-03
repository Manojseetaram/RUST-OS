

#![no_std] 
#![no_main] 
mod vga_buffer;
use core::panic::PanicInfo;

/// This function is called on panic.
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    println!("{}", _info);
    loop {}
}



#[no_mangle]
pub extern "C" fn _start() -> ! {
//    use core::fmt::Write;
//    vga_buffer::WRITER.lock().write_str("Rust is Not a Cult").unwrap();
//    write!(vga_buffer::WRITER.lock() , ", some numbers : {} {}" , 42 , 1.337).unwrap();
println!("Hello , Word {}", "!");
panic!("Some panic Messege");
    loop {}
}