use core::arch::asm;

pub mod gdt;

// pub fn outb(port: u16, data: u8) -> () {
// 	unsafe {
// 		asm!("outb al, dx", in("al")data, in("dx")port );
// 	}
// }

// pub fn inb(port: u16) -> u8 {
// 	unsafe {
// 		let ret: u8;
// 		asm!("inb dx, al", in("dx")port, out("al") ret);
// 		return ret;
// 	}
// }
