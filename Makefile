build:
	cargo build --release

install:
	cp target/release/gstest-kits /usr/bin/

bai:
	cargo build --release
	cp target/release/gstest-kits /usr/bin/

clean:
	rm -rf target