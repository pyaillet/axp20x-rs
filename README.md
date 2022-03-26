<p align="center">
    <a href="https://github.com/pyaillet/axp20x-rs/actions/workflows/ci.yml"><img src="https://github.com/pyaillet/axp20x-rs/actions/workflows/ci.yml/badge.svg?branch=main" alt="Build status" /></a>
    <a href="https://crates.io/crates/axp20x"><img src="https://img.shields.io/crates/v/axp20x.svg" alt="Crates.io"></a>
    <a href="https://docs.rs/axp20x"><img src="https://docs.rs/axp20x/badge.svg" alt="Docs.rs"></a>
</p>

# AXP20X Rust driver

Minimal Axp20x implementation.

What's working:
- Changing Power modes
- Configuring interrupts
- Checking interrupt state

Interrupt service handler setup is not provided, as it depends on your platform

## Examples

You can find an example usage in this project: [TTGO T-Watch v1 rust example](https://github.com/pyaillet/twatch-idf-rs).

## Contributing

This project is open to contributions of any form, do not hesitate to open an issue or a pull-request
if you have questions or suggestions.
