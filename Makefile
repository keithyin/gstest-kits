build:
	cargo build --release

install:
	cargo build --release
	cp target/release/gstest-kits /usr/bin/

clean:
	rm -rf target