curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
sudo apt-get install nasm xorriso qemu build-essentialrustup update nightly
rustup update nightly
rustup default nightly
rustup component add rust-src