# RISC-V Bare Metal

- [ ] Install the RISC-V Rust target.
- [ ] Build the freestanding binary with rustc.
- [ ] Confirm the main executable was created.
- [ ] Install QEMU RISC-V support.
- [ ] Run the binary in QEMU with the sifive_u machine.
- [ ] Use -bios none to avoid firmware overlap.
- [ ] Expect no output because the kernel loops forever.

`rustup target add riscv64imac-unknown-none-elf`

`rustc src/main.rs -C panic=abort -C linker=rust-lld -C "link-args=-e _start -Tmemory.x -Tlink.x" --target riscv64imac-unknown-none-elf`

`qemu-system-riscv64 -machine sifive_u -bios none -kernel main`
