.PHONY: run clean

rgraph.com: src/main.rs Cargo.toml link.x i586-rust_dos.json
	cargo build --release
	cargo objcopy --release -- -O binary --binary-architecture=i386:x86 rgraph.com

run: rgraph.com
	dosbox ./rgraph.com

clean:
	cargo clean
	rm -f rgraph.com
