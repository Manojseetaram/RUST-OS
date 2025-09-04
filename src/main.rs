
#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(blog_os::test_runner)]
#![reexport_test_harness_main = "test_main"]

use core::panic::PanicInfo;
use blog_os::println;
use bootloader::{BootInfo, entry_point};
entry_point!(kernel_main);
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
fn kernel_main(boot_info: &'static BootInfo) -> ! {
     use blog_os::memory::active_level_4_table;
    use x86_64::VirtAddr;

    println!("Rust is  Not a cult{}", "!");
   blog_os::init();

//       let ptr = 0x2044fc as *mut u8;
//       unsafe { let x = *ptr; }
//      println!("read worked");

//   unsafe { *ptr = 42; }
// println!("write worked");
//paging implmentaion
 let phys_mem_offset = VirtAddr::new(boot_info.physical_memory_offset);
    let l4_table = unsafe { active_level_4_table(phys_mem_offset) };

    for (i, entry) in l4_table.iter().enumerate() {
        if !entry.is_unused() {
            println!("L4 Entry {}: {:?}", i, entry);
        }
    }

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

