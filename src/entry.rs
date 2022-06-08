#![feature(start)]
#![no_std]
#![no_main]

mod cpu;
mod drivers;

use core::panic::PanicInfo;
use drivers::screen::{colours, print::kprint};

#[no_mangle]
pub extern "C" fn kmain() -> ! {
	unsafe{ init(); }

	// cpu::outb(0x3D4, 0x0A);
	// cpu::outb(0x3D5, 0x20);

	loop {}
}

#[panic_handler]
pub extern "C" fn kpanic(_info: &PanicInfo) -> ! {
	unsafe {
		*(0xB9000 as *mut u8) = 0x50;
		*(0xB9001 as *mut u8) = 0x50;
	}
	loop {}
}

unsafe fn init() {
	drivers::screen::print::init();

	kprint("Eisen OS - Running Checks...\n", colours::OUT);

	kprint("Loading GDT...\n", colours::PROC);
	cpu::gdt::gdt_init();
	kprint("Loaded GDT\n", colours::OK);

	kprint("All checks passed!\n", colours::OK);
}
