build:
	cargo build -r

install: build
	cp target/release/flatten /usr/local/bin/flatten