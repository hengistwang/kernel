#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(kernel::test_runner)]
#![reexport_test_harness_main = "test_main"]

extern crate alloc;

use alloc::boxed::Box;
use alloc::rc::Rc;
use alloc::vec;
use alloc::vec::Vec;
use bootloader::BootInfo;
use bootloader::entry_point;
use core::panic::PanicInfo;
use kernel::allocator;
use kernel::memory;
use kernel::memory::BootInfoFrameAllocator;
use kernel::println;
use x86_64::VirtAddr;

entry_point!(kernel_main);

fn kernel_main(_boot_info: &'static BootInfo) -> ! {
    kernel::init();

    let phys_mem_offset = VirtAddr::new(_boot_info.physical_memory_offset);
    let mut mapper = unsafe { memory::init(phys_mem_offset) };
    let mut frame_allocator = unsafe { BootInfoFrameAllocator::init(&_boot_info.memory_map) };

    allocator::init_heap(&mut mapper, &mut frame_allocator).expect("heap initialization failed");

    let x = Box::new(123);
    println!("{:p}", x);
    let mut v = Vec::new();
    for i in 0..500 {
        v.push(i);
    }
    println!("v as {:p}", v.as_slice());

    let reference_counter = Rc::new(vec![1, 2, 3]);
    let cloned_reference = reference_counter.clone();
    println!("{}", Rc::strong_count(&cloned_reference));
    core::mem::drop(reference_counter);
    println!("{}", Rc::strong_count(&cloned_reference));

    #[cfg(test)]
    test_main();
    kernel::hlt_loop();
}

#[cfg(not(test))]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    kernel::hlt_loop();
}

#[cfg(test)]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    kernel::test_panic_handler(info)
}
