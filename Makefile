build:
	cargo build -r

install: build
	cp target/release/flatten /usr/local/bin/flatten

clean:
	cargo clean

uninstall:
	rm /usr/local/bin/flatten