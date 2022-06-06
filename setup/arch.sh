curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
sudo pacman -S --needed binutils grub mtools libisoburn nasm qemu
rustup update nightly
rustup default nightly
cargo install xargo
rustup component add rust-src
cargo build -Zbuild-std