build:
	cargo build --release
install:
	cp src/kat.1.man /usr/share/man/man1/kat.1
	cp target/release/kat /usr/bin/kat
uninstall:
	rm /usr/share/man/man1/kat.1
	rm /usr/bin/kat
clean:
	rm -rf target
