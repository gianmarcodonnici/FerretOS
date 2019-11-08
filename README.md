# FerretOS
Hobby OS written in Rust.

To build a bootable kernel image:
cargo bootimage

To build and run an image on qemu:
cargo xrun

To run tests:
cargo xtest

Tools used:
- Rust
- bootimage crate
- cargo-xbuild crate
- QEMU

## Setting up the dev environment:
Run:
 - cargo install cargo-xbuild
 - cargo install bootimage
 - rustup component add llvm-tools-preview

Make sure you are using rust nightly

# TODO List
- Write a bootloader (preferably UEFI)
- Handle more cpu exceptions
- APIC

# Tutorials used
 - os.phil-opp.com
