global start

extern long_mode_start

section .boot
bits 32
start:
    mov esp, stack_top

    ; setup paging
    call setup_page_tables
    call enable_paging

    lgdt [gdt64.pointer]

    ; update selectors
    mov ax, gdt64.data
    mov ss, ax
    mov ds, ax
    mov es, ax

    ; long jump to long_mode_start setting `cs` register to `gdt64.code`
    jmp gdt64.code:long_mode_start

    ; shouldn't ever happen
    hlt

bits 32
error:
    mov dword [0xb8000], 0x4f524f45
    mov dword [0xb8004], 0x4f3a4f52
    mov dword [0xb8008], 0x4f204f20
    mov byte  [0xb800a], al

setup_page_tables:
    mov eax, p3_table				; copy first level 3 entry
	or eax, 0b11					; byte is in memory and may be written to
	mov dword [p4_table + 0], eax	; point level 4 table to level 3

	mov eax, p2_table				; copy first level 2 entry
	or eax, 0b11					; byte is in memory and may be written to
	mov dword [p3_table + 0], eax	; point level 3 table to level 2

    mov ecx, 0
	.map_p2_table:
		; map ecx-th P2 entry to a huge page that starts at address 2MiB*ecx
		mov eax, 0x200000  ; 2MiB
		mul ecx            ; result stored in eax
		or eax, 0b10000011 ; we're using 2MiB
		mov [p2_table + ecx * 8], eax

		inc ecx            ; increase counter
		cmp ecx, 512       ; if counter == 512, the whole P2 table is mapped
		jne .map_p2_table  ; else map the next entry

    ret


enable_paging:
    ; load paging
    mov eax, p4_table
    mov cr3, eax

    ; enable PAE (physical address extension)
    mov eax, cr4
    or eax, 1 << 5
    mov cr4, eax

	; set LME (long mode enable)
    mov ecx, 0xC0000080
    rdmsr
    or eax, 1 << 8
    wrmsr

    ; enable paging in the cr0 register
    mov eax, cr0
    or eax, (1 << 31) | (1 << 16)
    mov cr0, eax

    ret

section .bss
align 4096
p4_table:
    resb 4096
p3_table:
    resb 4096
p2_table:
    resb 4096
stack_bottom:
    resb 16384
stack_top:

%include "gdt.asm"

