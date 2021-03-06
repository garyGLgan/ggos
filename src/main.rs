#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(ggos::test_runner)]
#![reexport_test_harness_main = "test_main"]

extern crate alloc;
use alloc::{boxed::Box, rc::Rc, vec, vec::Vec};
use bootloader::{entry_point, BootInfo};
use core::panic::PanicInfo;
use ggos::println;
use ggos::task::{Task, simple_executor::SimpleExecutor, executor::Executor, keyboard};

entry_point!(kernel_main);

fn kernel_main(boot_info: &'static BootInfo) -> ! { 
    use ggos::allocator;
    use ggos::memory;
    use ggos::memory::BoolInfoFrameAllocator;
    use ggos::pci::scan_devices;
    use x86_64::VirtAddr;

    println!("Hello World{}", "!");

    ggos::init();

    let phys_mem_offset = VirtAddr::new(boot_info.physical_memory_offset);

    let mut mapper = unsafe { memory::init(phys_mem_offset) };
    let mut frame_allocator = unsafe { BoolInfoFrameAllocator::init(&boot_info.memory_map) };

    allocator::init_heap(&mut mapper, &mut frame_allocator).expect("heap initialization failed");

    scan_devices();

    let mut executor = Executor::new();
    executor.spawn(Task::new(example_task()));
    executor.spawn(Task::new(keyboard::print_keypresses()));
    executor.run();

    #[cfg(test)]
    test_main();

   

    println!("It did not crash!");
    ggos::hlt_loop();
}

async fn async_number() -> u32 {
    42
}

async fn example_task() {
    let number = async_number().await;
    println!("async number: {}", number);
}

#[cfg(not(test))]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    ggos::hlt_loop();
}

#[cfg(test)]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    ggos::test_panic_handler(info);
}
