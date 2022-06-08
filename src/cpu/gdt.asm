bits 64

gdtr:
	dw 0
	dq 0

global gdt_load
gdt_load:
	mov [gdtr], di
	mov [gdtr+2], rsi
	lgdt [gdtr]
	
	push 0x08
	lea rax, [rel reload_cs]
	push rax

	lretq

reload_cs:
	mov ax, 0x10
	mov ds, ax
	mov es, ax
	mov fs, ax
	mov gs, ax
	mov ss, ax
	ret