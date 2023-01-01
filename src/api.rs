use core::panic::PanicInfo;

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
  loop {}
}

#[no_mangle]
#[lang = "eh_personality"]
pub extern "C" fn rust_eh_personality()
{}
