#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(toni_os::test_runner)]
#![reexport_test_harness_main = "test_main"]

use core::panic::PanicInfo;
use toni_os::println;

#[no_mangle]
pub extern "C" fn _start() -> ! {
    println!("Hello World{}", "!");

    toni_os::init();

    #[cfg(test)]
    test_main();

    // // 故意触发 page fault
    // let ptr = 0xdeadbeaf as *mut u8;
    // unsafe { *ptr = 42; }

    // 查看内核中页表的存储方式
    use x86_64::registers::control::Cr3;
    let (level_4_page_table, _) = Cr3::read();
    println!("Level 4 page table at: {:?}", level_4_page_table.start_address());

    println!("It did not crash!");
    toni_os::hlt_loop();
}

/// This function is called on panic.
#[cfg(not(test))]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    toni_os::hlt_loop();
}

#[cfg(test)]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    toni_os::test_panic_handler(info)
}

#[test_case]
fn trivial_assertion() {
    assert_eq!(1, 1);
}
