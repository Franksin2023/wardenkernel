#![no_std]
#![no_main]

use core::panic::PanicInfo;
use wardenkernel::{adversarial, capability};

static HELLO: &[u8] = b"agent-kernel: boot ok";
static CAP_PASS: &[u8] = b"capability_tests: PASS";
static CAP_FAIL: &[u8] = b"capability_tests: FAIL";
static ADV_PASS: &[u8] = b"adversarial_proof: PASS";
static ADV_FAIL: &[u8] = b"adversarial_proof: FAIL";

fn write_vga(row: isize, msg: &[u8], is_ok: bool) {
    let vga_buffer = 0xb8000 as *mut u8;
    let offset = row * 160;
    for (i, &byte) in msg.iter().enumerate() {
        unsafe {
            *vga_buffer.offset(offset + i as isize * 2) = byte;
            *vga_buffer.offset(offset + i as isize * 2 + 1) = if is_ok { 0xa } else { 0xc };
        }
    }
}

#[no_mangle]
pub extern "C" fn _start() -> ! {
    let vga_buffer = 0xb8000 as *mut u8;
    for (i, &byte) in HELLO.iter().enumerate() {
        unsafe {
            *vga_buffer.offset(i as isize * 2) = byte;
            *vga_buffer.offset(i as isize * 2 + 1) = 0xb;
        }
    }

    let cap_passed = capability::run_tests();
    write_vga(1, if cap_passed { CAP_PASS } else { CAP_FAIL }, cap_passed);

    let adv_passed = adversarial::run_adversarial_tests();
    write_vga(2, if adv_passed { ADV_PASS } else { ADV_FAIL }, adv_passed);

    loop {}
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}
