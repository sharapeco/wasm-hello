#![no_main]
#![no_std]

#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! {
	loop {}
}

// JavaScript の関数をインポート
#[link(wasm_import_module = "imports")]
extern {
	fn math_sin(x: f64) -> f64;
	fn math_cos(x: f64) -> f64;
}

#[no_mangle]
pub fn add(a: i32, b: i32) -> i32 {
	a + b
}

#[no_mangle]
pub fn sin(x: f64) -> f64 {
	unsafe { math_sin(x) }
}

#[no_mangle]
pub fn cos(x: f64) -> f64 {
	unsafe { math_cos(x) }
}
