#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(ggos::test_runner)]
#![reexport_test_harness_main = "test_main"]

extern crate alloc;

use alloc::boxed::Box;
use alloc::vec::Vec;
use bootloader::{entry_point, BootInfo};
use core::panic::PanicInfo;
use ggos::allocator::HEAP_SIZE;
use ggos::{serial_print, serial_println};

entry_point!(main);

fn main(boot_info: &'static BootInfo) -> ! {
    use ggos::allocator;
    use ggos::memory::{self, BoolInfoFrameAllocator};
    use x86_64::VirtAddr;

    ggos::init();
    let phys_mem_offset = VirtAddr::new(boot_info.physical_memory_offset);
    let mut mapper = unsafe { memory::init(phys_mem_offset) };
    let mut frame_allocator = unsafe { BoolInfoFrameAllocator::init(&boot_info.memory_map) };

    allocator::init_heap(&mut mapper, &mut frame_allocator).expect("heap initialization faield");

    test_main();
    loop {}
}

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    ggos::test_panic_handler(info)
}

#[test_case]
fn simple_allocation() {
    serial_print!("simple_allocation...");
    let heap_value = Box::new(41);
    assert_eq!(*heap_value, 41);
    serial_println!("[Ok]");
}

#[test_case]
fn large_vec() {
    serial_print!("large_vec...");
    let n = 100;
    let mut vec = Vec::new();
    for i in 0..n {
        vec.push(i);
    }

    assert_eq!(vec.iter().sum::<u64>(), (n - 1) * n / 2);
    serial_println!("[Ok]");
}

#[test_case]
fn many_boxes() {
    serial_print!("many_boxes..");
    for i in 0..HEAP_SIZE {
        let x = Box::new(i);
        assert_eq!(*x, i);
    }
    serial_println!("[Ok]");
}

#[test_case]
fn many_boxes_long_lived() {
    serial_print!("many_boxes_long_lived...");
    let x = Box::new(1);
    for i in 0..HEAP_SIZE {
        let x = Box::new(i);
        assert_eq!(*x, i);
    }
    assert_eq!(*x, 1);
    serial_println!("[Ok]");
}
