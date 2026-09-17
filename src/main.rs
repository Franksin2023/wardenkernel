#![no_std]
#![no_main]

use core::panic::PanicInfo;
use wardenkernel::capability;

static HELLO: &[u8] = b"agent-kernel: boot ok";
static TEST_OK: &[u8] = b"capability_tests: PASS";
static TEST_FAIL: &[u8] = b"capability_tests: FAIL";

#[no_mangle]
pub extern "C" fn _start() -> ! {
    let vga_buffer = 0xb8000 as *mut u8;
    for (i, &byte) in HELLO.iter().enumerate() {
        unsafe {
            *vga_buffer.offset(i as isize * 2) = byte;
            *vga_buffer.offset(i as isize * 2 + 1) = 0xb;
        }
    }

    let test_passed = capability::run_tests();
    let msg = if test_passed { TEST_OK } else { TEST_FAIL };

    // Print test result on row 2 of VGA text buffer (offset 80 * 2 = 160)
    for (i, &byte) in msg.iter().enumerate() {
        unsafe {
            *vga_buffer.offset(160 + i as isize * 2) = byte;
            *vga_buffer.offset(160 + i as isize * 2 + 1) = if test_passed { 0xa } else { 0xc };
        }
    }

    loop {}
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}
