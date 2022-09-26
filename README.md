# bl702-hal

Hardware Abstraction Layer for BL702 RISC-V SoC in embedded Rust.

## Project status

Basic GPIO is working  
Everything else is missing

## Getting started

Install cargo + rust +riscv32imac-unknown-none-elf + cargo-binutils + llvm-tools-preview  
(google for instructions, TODO add step-by-step here)

```system
pip install bflb-mcu-tool

cargo objcopy --release --example blinky -- -O binary blinky.bin
bflb-mcu-tool --chipname bl702 --firmware blinky.bin
```

## Contributing

We welcome community contributions to this project. 
Please create a github issue or pull request if you have 
any issues wish to contribute.

## License

This project is licensed under both MIT or Mulan PSL v2