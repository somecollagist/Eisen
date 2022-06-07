#![feature(start)]
#![no_std]
#![no_main]

mod cpu;
mod screen;

use core::panic::PanicInfo;
use screen::{colours, print::kprint};

#[no_mangle]
pub extern "C" fn kmain() -> ! {
	unsafe {
		init();
	}
	kprint("Eisen OS - Running Checks...\n", colours::OUT);

	kprint("Loading GDT...\n", colours::PROC);
	cpu::gdt::init();
	kprint("Loaded GDT\n", colours::OK);

	kprint("EISEN> \n", colours::QUERY);
	kprint("OS\n", colours::ERR);

	loop {}
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
	unsafe {
		*(0xB9000 as *mut u8) = 0x50;
		*(0xB9001 as *mut u8) = 0x50;
	}
	loop {}
}

unsafe fn init() {
	screen::print::init();
	cpu::gdt::init();
}
