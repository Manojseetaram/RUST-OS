
#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(blog_os::test_runner)]
#![reexport_test_harness_main = "test_main"]

use core::panic::PanicInfo;
use blog_os::println;

#[unsafe(no_mangle)]
pub extern "C" fn _start() -> ! {
    println!("Rust is  Not a cult{}", "!");
   blog_os::init();
   blog_os::hlt_loop();  
    println!("It did not crash!");
//    x86_64::instructions::interrupts::int3();
//trigger page fault
// unsafe {
//    *(0xdeadbeef as *mut u64) = 42;
// }
// fn stack_overflow(){
//     stack_overflow();   
// }
// stack_overflow();
    #[cfg(test)]
    test_main();
 
    
    loop {
        use blog_os::print;
        print!("-");        
    }
       
}

/// This function is called on panic.
#[cfg(not(test))]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    blog_os::hlt_loop();            // new
}

#[cfg(test)]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    blog_os::test_panic_handler(info)
}