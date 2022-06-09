bits 64

%include "macros.asm"

; IRQ number, ISR number remap
%macro IRQ 2
global irq%1
irq%1:
	cli
	push byte 0
	push byte %2
	jmp irq_common_stub
%endmacro

IRQ	00, 0x20
IRQ	01, 0x21
IRQ	02, 0x22
IRQ	03, 0x23
IRQ	04, 0x24
IRQ	05, 0x25
IRQ	06, 0x26
IRQ	07, 0x27
IRQ	08, 0x28
IRQ	09, 0x29
IRQ	0A, 0x2A
IRQ	0B, 0x2B
IRQ	0C, 0x2C
IRQ	0D, 0x2D
IRQ	0E, 0x2E
IRQ	0F, 0x2F

extern kpanic
extern irq_handler

irq_common_stub:
	; mov ax, ds
	; push eax		;data segment
	; mov ax, 0x10	;kernel data
	; mov ds, ax
	; mov es, ax
	; mov fs, ax
	; mov gs, ax
	; call irq_handler
	; pop ebx			;reload original data segment
	; mov ds, bx
	; mov es, bx
	; mov fs, bx
	; mov gs, bx
	; add esp, 8
	; sti
	iretq

