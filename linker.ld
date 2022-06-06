ENTRY(start)

SECTIONS {
	. = 1M;

	.boot :
	{
		*(.multiboot_header)
	}

	.text :
	{
		*(.boot)
		*(.text)
	}
}