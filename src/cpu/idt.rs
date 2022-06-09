use crate::cpu::{outb};
use crate::drivers::screen::{print, colours};

#[repr(C, packed)]
struct IDTEntry{
	offsetlow:	u16,
	selector:	u16,
	ist_resv:	u8,		//only the low 3 bits are used for IST, the rest are reserved
	flags:		u8,
	offsetmid:	u16,
	offsethigh:	u32,
	reserved:	u32
}

extern "C" {
	static mut _idt:	[IDTEntry; 256];
	static isr1:		u64;
	fn load_idt();
}

pub fn idt_init(){
	unsafe{
		let isrptr = (&isr1 as *const _) as u64;

		for x in 0..256{
			let x = x as usize;
	
			_idt[x].offsetlow 	= isrptr as u16;
			_idt[x].selector 	= 0x08;
			_idt[x].ist_resv 	= 0;
			_idt[x].flags 		= 0x8E;
			_idt[x].offsetmid 	= (isrptr >> 16) as u16;
			_idt[x].offsethigh 	= (isrptr >> 32) as u32;
			_idt[x].reserved	= 0;
		}
		
		outb(0x21, 0xFD);
		outb(0xA1, 0xFF);
		load_idt();
	}
}

#[no_mangle]
pub extern "C" fn isr1_handler(){
	print::kprint("?", colours::ERR);

	outb(0x20, 0x20);
	outb(0xA0, 0x20);
}