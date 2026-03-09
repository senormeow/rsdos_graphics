# Mode 13h Graphics Demo in Rust

A bare-metal Rust program that draws VGA graphics in real mode DOS using Mode 13h (320x200, 256 colors). Compiles to a tiny `.COM` binary.

## Requirements

- Rust nightly toolchain
- `cargo-binutils` (`cargo install cargo-binutils`)
- DOSBox (for running)

## Build

```
make
```

## Run

```
make run
```

## Clean

```
make clean
```
