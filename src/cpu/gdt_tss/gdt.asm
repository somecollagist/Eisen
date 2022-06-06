bits 64

global gdt64_load
global gdt64_reload
global tss64_flush

gdt64_load:
	lgdt [rdi]

	push rbp
	mov rbp, rsp

	push rsi
	push rbp
	pushfq

	push rdx
	push .reload

	iretq

.reload:
	pop rbp
	mov ss, rsi
	mov gs, rsi
	mov fs, rsi
	mov ds, rsi
	mov es, rsi
	ret

tss64_flush:
	mov ax, 0x28

	ltr ax

	ret