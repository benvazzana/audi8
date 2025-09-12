# audi8
[![Build](https://github.com/benvazzana/audi8/actions/workflows/rust.yml/badge.svg)](https://github.com/benvazzana/audi8/actions/workflows/rust.yml)

An audio transposition tool (WIP)

---

## Usage
Build using `cargo`:
```sh
cargo build
```

To run the CLI (debug build):
```sh
cargo run <file> <num-semitones>
```
Arguments:
- path to a WAV file
- number of semitones to transpose (must be in range [-12, 12])
