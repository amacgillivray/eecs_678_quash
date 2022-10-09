quash:
	cargo build
	cp ./target/debug/quash ./quash

test:
	cargo run

clean:
	rm -rf quash target/*