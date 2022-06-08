use core::arch::asm;
use core::mem::size_of;

#[repr(C, packed)]
#[derive(Clone, Copy)]
struct IDTEntry{
	offsetlow:	u16,
	selector:	u16,
	ist_resv:	u8,		//only the low 3 bits are used for IST, the rest are reserved
	flags:		u8,
	offsetmid:	u16,
	offsethigh:	u32,
	reserved:	u32
}

const fn new_idtentry(offset: u64, ist: u8, flags: u8) -> IDTEntry{
	IDTEntry {
		offsetlow:	(offset as u16),
		selector:	0x08,
		ist_resv:	ist,
		flags:		(flags | 0b1000000),	//Bit 8 must be 1 in order to be valid
		offsetmid:	((offset >> 16) as u16),
		offsethigh:	((offset >> 32) as u32),
		reserved:	0
	}
}

#[repr(C, packed)]
pub struct IDTDesc{
	limit:		u16,
	offset:		u64
}

static mut idt: [IDTEntry; 256] = [new_idtentry(0, 0, 0); 256];

extern "C" {
	static int_vt: [u64; 48];
}

pub fn set_idtentry(idx: usize, offset: u64, ist: u8, flags: u8){
	unsafe{
		idt[idx] = IDTEntry{
			offsetlow:	(offset as u16),
			selector:	0x08,
			ist_resv:	ist,
			flags:		flags,
			offsetmid:	((offset >> 16) as u16),
			offsethigh:	((offset >> 32) as u32),
			reserved:	0
		}
	}
}

pub fn idt_init(){
	unsafe{
		let mut idt_desc: IDTDesc = IDTDesc{
			limit:	(((size_of::<IDTEntry>() * 256) - 1) as u16),
			offset:	(&idt as *const _) as u64
		};

		for i in 0..48{
			set_idtentry(i as usize, int_vt[i as usize], 0, 0x8E)
		}

		asm!("lidt {idtd}", idtd = in(reg)((&idt_desc as *const _) as u64));
	}
}