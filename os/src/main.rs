#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(os::test_runner)]
#![reexport_test_harness_main = "test_main"]
#![feature(asm)] // to use software interrupt

use core::panic::PanicInfo;
use os::println;
use bootloader::{BootInfo, entry_point};
entry_point!(kernel_main);

fn kernel_main(boot_info : &'static BootInfo) -> ! {
    println!("Hello World{}", "!");
    println!("{:#?}", boot_info.memory_map.len());

    os::init();
    use x86_64::registers::control::Cr3;
    let (level_4_page_table, _) = Cr3::read();
    println!("Level 4 page table at: {:?}", level_4_page_table.start_address());
    let x = 5111;
    println!("{}",x);

    println!("{:#?}", x86_64::registers::control::Cr2::read());
    unsafe{
        x86_64::software_interrupt!(0x80);
    }
    println!("{}",x);

    // x86_64::instructions::interrupts::int3();
    // invoke a breakpoint exception
    // x86_64::instructions::interrupts::int3();

    #[cfg(test)]
    test_main();
//     let ptr = 0x204c8f as *mut u32;
// // read from a code page
//     unsafe {
//         let x = *ptr;
//         println!("read worked {:#?}", x);
//     }

// write to a code page
//     unsafe { *ptr = 42; }
//     println!("write worked");
//     println!("It did not crash!");
    os::hlt_loop();
}

/// This function is called on panic.
#[cfg(not(test))]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    os::hlt_loop();
}

#[cfg(test)]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    os::test_panic_handler(info)
}

#[test_case]
fn trivial_assertion() {
    assert_eq!(1, 1);
}