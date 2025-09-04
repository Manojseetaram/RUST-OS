
#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(blog_os::test_runner)]
#![reexport_test_harness_main = "test_main"]

use core::panic::PanicInfo;
use blog_os::println;
pub trait Testable {
    fn run(&self);
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
#[unsafe(no_mangle)]
pub extern "C" fn _start() -> ! {
    println!("Rust is  Not a cult{}", "!");
   blog_os::init();

//       let ptr = 0x2044fc as *mut u8;
//       unsafe { let x = *ptr; }
//      println!("read worked");

//   unsafe { *ptr = 42; }
// println!("write worked");

    use x86_64::registers::control::Cr3;

    let (level_4_page_table, _) = Cr3::read();
    println!("Level 4 page table at: {:?}", level_4_page_table.start_address());
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
  println!("It did not crash!");
    
   blog_os::hlt_loop()
       
}

