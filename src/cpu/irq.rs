use crate::drivers::screen::{print, colours};

#[repr(C, packed)]
struct interrupt_stack{
	r15:	u64,
	r14:	u64,
	r13:	u64,
	r12:	u64,
	r11:	u64,
	r10:	u64,
	r9:		u64,
	r8: 	u64,
	rbp:	u64,
	rdi:	u64,
	rsi:	u64,
	rdx:	u64,
	rcx:	u64,
	rbx:	u64,
	rax:	u64,

	int_no:	u64,
	err:	u64,

	rip:	u64,
	cs:		u64,
	rflags:	u64,
	rsp:	u64,
	ss:		u64
}

enum int_code{
	ZERO_DIVISION,
	DEBUG,
	NON_MASKABLE,
	BREAKPOINT,
	OVERFLOW,
	BOUND_RANGE_EXCEEDED,
	INVALID_OPCODE,
	DEV_NOT_AVAILABLE,
	DOUBLE_FAULT,
	COPROCESSOR_SEG_OVERRUN,
	INVALID_TSS,
	SEG_NOT_PRESENT,
	STACK_SEG_FAULT,
	GENERAL_PROTECTION_FAULT,
	PAGE_FAULT,
	X87_FPU_ERROR = 16,
	ALIGNMENT_CHECK,
	SIMD_FP_ERROR,
	VIRT_ERROR,
	CONTROL_PROTECTION_ERROR,
	PIT = 32,
	KEYBOARD,
	CASCADE,
	COM2,
	COM1,
	LPT2,
	FLOPPY,
	LPT1,
	CMOS,
	MOUSE = 44,
	FPU,
	PRIMARY_ATA,
	SECONDARY_ATA
}

#[no_mangle]
pub extern "C" fn irq_handler(frame: *const interrupt_stack){
	kprint("ISR handler", colours::ERR);
	if (*frame).int_no == int_code.KEYBOARD{
		
	}
}