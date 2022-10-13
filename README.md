# bl702-hal

Hardware Abstraction Layer for BL702 RISC-V SoC in embedded Rust.

## Project status

Working:
- Basic GPIO is working
- Clock initialisation (not configurable)
- UART0 (2MBaud only)

TODO:
- Everything else

## Getting started

Install cargo + rust + riscv32imac-unknown-none-elf + cargo-binutils + llvm-tools-preview  
(google for instructions, TODO add step-by-step here)

```system
pip install bflb-mcu-tool

cargo objcopy --release --example blinky -- -O binary blinky.bin
bflb-mcu-tool --chipname bl702 --firmware blinky.bin
```

Serial demo
```system
cargo objcopy --release --example serial -- -O binary serial.bin
bflb-mcu-tool --chipname bl702 --firmware serial.bin
picocom --lower-rts --lower-dtr --imap lfcrlf /dev/ttyUSB0 -b 2000000
```

## Contributing

We welcome community contributions to this project. 
Please create a github issue or pull request if you have 
any issues or wish to contribute.

## License

This project is licensed under both MIT or Mulan PSL v2