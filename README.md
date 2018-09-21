# ARM semihosting for AArch64

## Description

This crate provides a safe Rust wrapper for the [ARM semihosting interface][sh].

Compared to the [cortex-m-semihosting] library, this crate is focused on providing
support for ARMv8 Cortex-A processors, running in the AArch32 or AArch64 mode.

[sh]: https://developer.arm.com/docs/100863/latest
[cortex-m-semihosting]: https://github.com/rust-embedded/cortex-m-semihosting

## License

This repository is licensed under the Mozilla Public License Version 2.0.
See the [LICENSE.txt] file for the full text.
