// Credit to https://github.com/ackOS-project/ackOS/tree/master/kernel/arch/x86_64/feat

use std::mem;

#[repr(C, packed)]
pub enum GDT_OPTIONS {
	GDT_LONG_MODE_GRANULARITY 	= 0x20,
	GDT_SEGMENT 				= 0x10,
	GDT_PRESENT 				= 0x80,
	GDT_TSS_PRESENT 			= 0x80,
	GDT_USER 					= 0x60,
	GDT_EXECUTABLE 				= 0x08,
	GDT_READ_WRITE 				= 0x02,
	GDT_FLAGS 					= 0x0C,
}

static GDT_ENTRIES: u16 = 6;

#[repr(C, packed)]
pub struct GDT_STRUCT {
	gdt: 						[GDT_ENTRY; GDT_ENTRIES],
	tss: 						TSS_ENTRY,
}

#[repr(C, packed)]
pub struct GDT_ENTRY {
	limit_low: 					u16,
	base_low: 					u16,
	base_mid: 					u8,
	access: 					u8,
	granularity: 				u8,
	base_high: 					u8,
}

#[repr(C, packed)]
pub struct GDT_DESCRIPTOR {
	segment: 					u16,
	offset: 					u64,
}

#[repr(C, packed)]
pub struct TSS_STRUCT {
	reserved0: 					u32,
	rsp: 						[u64; 3],
	reserved1: 					u64,
	ist: 						[u64; 7],
	reserved2: 					u32,
	reserved3: 					u32,
	reserved4: 					u16,
	iomap_base: 				u16,
}

#[repr(C, packed)]
pub struct TSS_ENTRY {
	size: 						u16,
	base_low: 					u16,
	base_mid: 					u8,
	flags1: 					u8,
	flags2: 					u8,
	base_high: 					u8,
	base_upper: 				u32,
	reserved0: 					u32,
}

impl Default for TSS_ENTRY {
	fn default() -> TSS_ENTRY {
		TSS_ENTRY {
			size: 				0,
			base_low: 			0,
			base_mid: 			0,
			flags1: 			0,
			flags2: 			0,
			base_high: 			0,
			base_upper: 		0,
			reserved0: 			0,
		}
	}
}

static _gdt: GDT_STRUCT;
static _gdt_descriptor: GDT_DESCRIPTOR = GDT_DESCRIPTOR {
	segment:		mem::sizeof::<GDT_STRUCT> - 1,
	offset:			&_gdt as u64
};
static _tss: TSS_STRUCT = TSS_STRUCT {
	reserved0:		0,
	rsp:			{},
	reserved1:		0,
	ist:			{},
	reserved2:		0,
	reserved3:		0,
	reserved4:		0,
	iomap_base:		0
};

static KERNEL_CODE_SELECTOR:	u16 = 1;
static KERNEL_DATA_SELECTOR:	u16 = 2;
static USER_CODE_SELECTOR:		u16 = 4;
static USER_DATA_SELECTOR:		u16 = 5;

pub fn GDTInit() {
	// Null entry
	_gdt.gdt[0] = MakeGDTEntry(0, 0, 0, 0);

	_gdt.gdt[KERNEL_CODE_SELECTOR] = MakeGDTEntry(
		0,
		0,
		GDT_OPTIONS::GDT_LONG_MODE_GRANULARITY,
			GDT_OPTIONS::GDT_PRESENT |
			GDT_OPTIONS::GDT_SEGMENT |
			GDT_OPTIONS::GDT_READ_WRITE |
			GDT_OPTIONS::GDT_EXECUTABLE
	);

	_gdt.gdt[KERNEL_DATA_SELECTOR] = MakeGDTEntry(
		0, 
		0, 
		0,
			GDT_OPTIONS::GDT_PRESENT |
			GDT_OPTIONS::GDT_SEGMENT |
			GDT_OPTIONS::GDT_READ_WRITE
	);

	_gdt.gdt[3] = MakeGDTEntry(0, 0, 0, 0);

	_gdt.gdt[USER_CODE_SELECTOR] = MakeGDTEntry(
		0, 
		0, 
		GDT_OPTIONS::GDT_LONG_MODE_GRANULARITY, 
			GDT_OPTIONS::GDT_PRESENT |
			GDT_OPTIONS::GDT_SEGMENT |
			GDT_OPTIONS::GDT_READ_WRITE |
			GDT_OPTIONS::GDT_USER
	);

	_gdt.gdt[USER_DATA_SELECTOR] = MakeGDTEntry(
		0, 
		0, 
		0,
			GDT_OPTIONS::GDT_PRESENT |
			GDT_OPTIONS::GDT_SEGMENT |
			GDT_OPTIONS::GDT_READ_WRITE |
			GDT_OPTIONS::GDT_USER
	);

	_gdt.tss = MakeTSSEntry(&_gdt_descriptor);
}

pub fn MakeGDTEntry(base: u32, limit: u32, granularity: u8, access: u8) -> GDT_ENTRY {
	return GDT_ENTRY{
		base_low:	(base & 0xFFFF),
		base_mid:	((base >> 16) & 0xFF),
		base_high:	((base >> 24) & 0xFF),

		limit_low:	(limit & 0xFFFF),
		granularity:(((limit >> 16) & 0x0F)|(granularity & 0xF0)),

		access: 	access
	};
}

pub fn MakeTSSEntry(addr: u64) -> TSS_ENTRY {
	return TSS_ENTRY{
		size:		mem::sizeof::<TSS_STRUCT>,

		base_low:	((addr as u16) & 0xFFFF),
		base_mid:	(((addr >> 16) as u8) & 0xFF),
		base_high:	(((addr >> 24) as u8) & 0xFF),
		base_upper:	((addr >> 32) as u32),

		flags1:		0b10001001,
		flags2:		0,

		reserved0:	0
	}
}

pub fn TSSSetStack(addr: u64) {
	_tss.rsp[0] = addr;
	_tss.ist[0] = addr;
}

extern "C" {
	fn gdt64_load(descriptor: *const GDT_DESCRIPTOR, data: u64, code: u64);
	fn tss64_flush();
}

pub fn GDTLoad(descriptor: *const GDT_DESCRIPTOR) {
	gdt64_load(descriptor, KERNEL_DATA_SELECTOR * 8, KERNEL_CODE_SELECTOR * 8);
}

pub fn TSSFlush() {
	tss64_flush();
}