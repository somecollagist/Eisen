global isr1
global load_idt

extern _idt
extern isr1_handler

isr1:
	PUSHALL
	call isr1_handler
	POPALL
	iretq

idt_desc:
	dw 4095
	dq _idt

load_idt:
	lidt [idt_desc]
	sti
	ret