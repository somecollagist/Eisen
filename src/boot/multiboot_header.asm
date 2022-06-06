MAGIC	equ 0xE85250D6					; magic number
ARCH	equ 0							; use protected mode (that was easy!)
LENGTH	equ header_end - header_start	; header length

section .multiboot_header
header_start:
	dd MAGIC
	dd ARCH
	dd LENGTH

	; checksum
	dd 0x100000000 - (MAGIC + ARCH + LENGTH)

	; ending tags
	dw 0	; type
	dw 0	; flags
	dd 8	; size
header_end: