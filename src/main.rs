
#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(blog_os::test_runner)]
#![reexport_test_harness_main = "test_main"]

use core::panic::PanicInfo;
use blog_os::{ println};
use bootloader::{BootInfo, entry_point};
extern crate alloc;


use x86_64::VirtAddr;
use alloc::{boxed::Box, vec, vec::Vec, rc::Rc};
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
  use blog_os::allocator;
    use blog_os::memory::{self, BootInfoFrameAllocator};

    println!("Hello World{}", "!");
    blog_os::init();

    let phys_mem_offset = VirtAddr::new(boot_info.physical_memory_offset);
    let mut mapper = unsafe { memory::init(phys_mem_offset) };
    let mut frame_allocator = unsafe {
        BootInfoFrameAllocator::init(&boot_info.memory_map)
    };

   
    allocator::init_heap(&mut mapper, &mut frame_allocator)
        .expect("heap initialization failed");

    let x = Box::new(41);

 let heap_value = Box::new(41);
    println!("heap_value at {:p}", heap_value);

 let mut vec = Vec::new();
    for i in 0..500 {
        vec.push(i);
    }
    println!("vec at {:p}", vec.as_slice());
        let reference_counted = Rc::new(vec![1, 2, 3]);
    let cloned_reference = reference_counted.clone();
    println!("current reference count is {}", Rc::strong_count(&cloned_reference));
    core::mem::drop(reference_counted);
    println!("reference count is {} now", Rc::strong_count(&cloned_reference));
     #[cfg(test)]
    test_main();
    println!("It did not crash {x}!");
    
    blog_os::hlt_loop()
//mapo unused page 
// let page = Page::containing_address(VirtAddr::new(0xdeadbeaf000b8000));
// memory::create_example_mapping(page,&mut  mapper,&mut frame_allocater);

// //writing the string New! to the screen throught the new mapping

// let page_ptr : *mut u64 = page.start_address().as_mut_ptr();
// unsafe {page_ptr.offset(400).write_volatile(0x_f021_f077_f065_f04e)};
  
       
}

