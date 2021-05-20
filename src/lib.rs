#![no_main]

use std::f32;

// #[panic_handler]
// fn panic(_info: &core::panic::PanicInfo) -> ! {
// 	loop {}
// }

#[no_mangle]
pub fn add(a: i32, b: i32) -> i32 {
	a + b
}

#[no_mangle]
pub fn sin(x: f32) -> f32 {
	x.sin()
}
