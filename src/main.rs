
#![no_std] 
#![no_main] 

#![feature(custom_test_frameworks)]
#![test_runner(crate::test_runner)]
#![reexport_test_harness_main = "test_main"]
mod vga_buffer;
use core::panic::PanicInfo;
pub enum QeumExitCode {
    Success = 0x10,
    Failed = 0x11,
}
pub fn exit_qemu(exit_code : QeumExitCode){
    use x86_64::instructions::port::Port;
    unsafe {
        let mut port = Port::new(0xf4);
        port.write(exit_code as u32);
    }
}

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
#[cfg(test)]
test_main();

    loop {}
}


#[cfg(test)]
fn test_runner(tests : &[&dyn Fn()]){
   println!("Running {} tests" , tests.len() );
   for test in tests{
    test();
   }
}
#[test_case]
fn trivial_assertaion(){
    print!("taival assertion... ");
    assert_eq!(1 , 1);
    println!("[ok]")
}