use core::arch::asm;
use core::mem::size_of;

#[repr(C, packed)]
#[derive(Clone, Copy)]
struct GDTEntry{
	limitlow:	u16,
	baselow:	u16,
	basemid:	u8,
	access:		u8,
	limitflags:	u8,
	basehigh:	u8
}

fn newGDTEntry(base: u32, limit: u32, access: u8, flags: u8) -> GDTEntry{
	GDTEntry {
		limitlow:		(limit & 0xFFFF) 								as u16,
		baselow:		(base & 0xFFFF)									as u16,
		basemid:		((base >> 16) & 0xFF)							as u8,
		access:			(access)										as u8,
		limitflags:		(((limit >> 16) & 0xF) | (flags as u32 & 0xF0))	as u8,
		basehigh:		((base >> 24) & 0xFF)							as u8
	}
}

#[repr(C, packed)]
struct GDTDesc{
	limit:		u16,
	offset:		u64,
}

#[repr(C, packed)]
struct TSSEntry{
	size:		u16,
	baselow:	u16,
	basemid:	u8,
	flags1:		u8,
	flags2:		u8,
	basehigh:	u8,
	baseupper:	u32,
	reserved:	u32
}

// use u64 since it's a 64-bit OS (duh)
fn newTSSEntry(addr: u64) -> TSSEntry {
	TSSEntry{
		size:			size_of::<TSSStruct>() as u16,
		baselow:		((addr as u16) & 0xFFFF),
		basemid:		(((addr >> 16) as u8) & 0xFF),
		flags1:			0b10001001,
		flags2:			0b00000000,
		basehigh:		(((addr >> 24) as u8) & 0xFF),
		baseupper:		((addr >> 32) as u32),
		reserved:		0
	}
}

#[repr(C, packed)]
struct TSSStruct{
	reserved0:	u32,
	rsp:		[u64; 3],
	reserved1:	u64,
	ist:		[u64; 7],
	reserved2:	u32,
	reserved3:	u32,
	reserved4:	u16,
	iomap_base:	u16
}

const GDTENTRIES		: usize = 6;

#[repr(C, packed)]
struct g_GDT{
	gdt:		[GDTEntry; GDTENTRIES],
	tss:		TSSEntry,
}

const tss: TSSStruct = TSSStruct{
	reserved0:	0,
	rsp:		[0; 3],
	reserved1:	0,
	ist:		[0; 7],
	reserved2:	0,
	reserved3:	0,
	reserved4:	0,
	iomap_base:	0
};

extern "C" {
	fn tss_flush();
	fn gdt_load(limit: u16, offset: u64);
}

pub fn gdt_init(){
	unsafe{ asm!("cli"); }	//Disable interrupts

	let mut gdt: g_GDT = g_GDT {
		gdt: [
			newGDTEntry(0, 0, 0, 0);
			GDTENTRIES
			],
		tss: newTSSEntry((&tss as *const _) as u64)
	};

	let gdt_desc: GDTDesc = GDTDesc{
		limit:	((size_of::<g_GDT>() - 1) as u16),
		offset:	((&gdt as *const _) as u64)
	};

	//null descriptor
	gdt.gdt[0] = newGDTEntry(0, 0, 0, 0);
	//kernel code
	gdt.gdt[1] = newGDTEntry(0, 0xFFFFF, 0x9A, 0xA);
	//kernel data
	gdt.gdt[2] = newGDTEntry(0, 0xFFFFF, 0x92, 0xC);
	//extra null descriptor?
	gdt.gdt[3] = newGDTEntry(0, 0, 0, 0);
	//user code
	gdt.gdt[4] = newGDTEntry(0, 0xFFFFF, 0xFA, 0xA);
	//user data
	gdt.gdt[5] = newGDTEntry(0, 0xFFFFF, 0xF2, 0xC);

	unsafe{ gdt_load(gdt_desc.limit, gdt_desc.offset); }
}