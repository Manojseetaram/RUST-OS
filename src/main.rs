
#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(blog_os::test_runner)]
#![reexport_test_harness_main = "test_main"]

use core::panic::PanicInfo;
use blog_os::println;
use bootloader::{BootInfo, entry_point};
use x86_64::structures::paging::Page;

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
    

    use blog_os::memory;
    use x86_64::{structures::paging::Page, VirtAddr };
      
    println!("Rust is  Not a cult{}", "!");
   blog_os::init();



  let phys_mem_offset = VirtAddr::new(boot_info.physical_memory_offset);
  let mut mapper = unsafe { memory::init(phys_mem_offset) };
  let mut frame_allocater = memory::EmptyFrameAllocator;
   
//mapo unused page 
let page = Page::containing_address(VirtAddr::new(0));
memory::create_example_mapping(page,&mut  mapper,&mut frame_allocater);

//writing the string New! to the screen throught the new mapping

let page_ptr : *mut u64 = page.start_address().as_mut_ptr();
unsafe {page_ptr.offset(400).write_volatile(0x_f021_f077_f065_f04e)};
    #[cfg(test)]
    test_main();
  println!("It did not crash!");
    
   blog_os::hlt_loop()
       
}

