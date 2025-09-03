
#![no_std] 
#![no_main] 

#![feature(custom_test_frameworks)]
#![test_runner(crate::test_runner)]
#![reexport_test_harness_main = "test_main"]
mod vga_buffer;
mod serial;
use core::panic::PanicInfo;
pub enum QemuExitCode {
    Success = 0x10,
    Failed = 0x11,
}
pub fn exit_qemu(exit_code : QemuExitCode){
    use x86_64::instructions::port::Port;
    unsafe {
        let mut port = Port::new(0xf4);
        port.write(exit_code as u32);
    }
}


#[cfg(not(test))]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    loop {}
}
#[cfg(test)]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    serial_println!("[failed]\n");
    serial_println!("Error: {}\n", info);
    exit_qemu(QemuExitCode::Failed);
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
   serial_println!("Running {} tests" , tests.len() );
   for test in tests{
    test();
   }
   exit_qemu(QemuExitCode::Success);
}
#[test_case]
fn trivial_assertaion(){
    serial_print!("taival assertion... ");
    assert_eq!(1 , 1);
    serial_println!("[ok]");
    loop {
        
    }
}