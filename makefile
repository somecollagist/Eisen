SRC := ./src
BIN := ./target
ISO := ./iso
BINLOC := $(ISO)/boot/kernel.bin
TARGET := $(shell cat ./.cargo/config.toml | grep target | cut -d "/" -f2 | cut -d "." -f1)

ASM := nasm
ASMFLG := -f elf64
ASMSRC := $(shell find $(SRC) -name "*.asm")
ASMTAR := $(patsubst $(SRC)/%,$(BIN)/%,$(patsubst %.asm,%.o,$(ASMSRC)))
ASMIDR := $(shell dirname $(shell echo $(ASMSRC) | tr ' ' '\n' | sort -u | xargs))

ASMINC := $(addprefix -i ,$(ASMIDR))

all: prebuild build run clean

prebuild:
	clear
	rm -rf $(BIN)
	mkdir $(BIN)
	cargo clean

build: $(ASMTAR)
	~/.cargo/bin/xargo build -Zbuild-std
	ld --nmagic --output=$(BINLOC) --script=linker.ld $^ $(BIN)/$(TARGET)/debug/libeisen.a
	grub-mkrescue -o eisen.iso $(ISO)

$(BIN)/%.o: $(SRC)/%.asm
	mkdir -p $(shell dirname $@)
	$(ASM) $(ASMFLG) $< -o $@ $(ASMINC)

run:
	qemu-system-x86_64 -cdrom eisen.iso

clean:
	rm -f $(BINLOC)