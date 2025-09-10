#![no_std]
#![no_main]
#![allow(clippy::empty_loop)]

use core::panic::PanicInfo;

mod startup_stm32f303;

static mut SCORES_GLOBAL: [u32; 5] = [1, 2, 3, 4, 5]; // flash at > stack
const _NUMBERS: [u32; 5] = [10, 20, 30, 40, 50]; // flash
static mut BUFFER: [u8; 1024] = [0; 1024]; // flash at > stack

#[unsafe(no_mangle)]
fn main() {
    let mut _total_score = 0;

    unsafe {
        for score in SCORES_GLOBAL {
            _total_score += score;
        }
    }

    unsafe {
        BUFFER[0] = 100;
    }

    loop {}
}

#[panic_handler]
fn panic_handler(_info: &PanicInfo) -> ! {
    loop {}
}
