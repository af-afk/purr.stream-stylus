#![cfg_attr(not(any(test, feature = "export-abi")), no_main)]

#[cfg(not(any(test, feature = "export-abi")))]
#[no_mangle]
pub extern "C" fn main() {}

#[cfg(feature = "export-abi")]
fn main() {
    purrstreamstylus::print_abi("MIT-OR-APACHE-2.0", "pragma solidity ^0.8.23;");
}

#[panic_handler]
#[cfg(target_arch = "wasm32")]
fn panic(_: &core::panic::PanicInfo) -> ! {
    core::arch::wasm32::unreachable()
}
