use core::arch::asm;

pub mod gdt;
pub mod idt;

pub fn outb(port: u16, data: u8) {
	unsafe {
		asm!(
			"out dx, al",
			in("dx")port,
			in("al")data
		);
	}
}

pub fn inb(port: u16) -> u8 {
	unsafe {
		let mut ret: u8;
		asm!(
			"inb dx, al",
			in("dx")port,
			out("al")ret
		);
		return ret;
	}
}
