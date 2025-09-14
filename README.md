# Mode 13h Graphics Demo in Rust

Trying to display graphics in real mode DOS using Rust

```
cargo build --release
cargo objcopy --release -- -O binary --binary-architecture=i386:x86 rgraph.com
```
