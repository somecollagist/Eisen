#![feature(start)]
#![no_std]
#![no_main]

mod screen;

use core::panic::PanicInfo;

#[no_mangle]
pub extern "C" fn kmain() -> ! {
    screen::print::kprint(
        "Eisen OS",
        screen::colours::getcol(
            screen::colours::VGACOL::LGREY,
            screen::colours::VGACOL::BLACK,
        ),
    );
    loop {}
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}
