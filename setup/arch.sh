sudo pacman -S --needed binutils grub mtools libisoburn nasm qemu rustup
rustup update nightly
rustup default nightly
cargo install xargo
rustup component add rust-src
cargo build -Zbuild-std