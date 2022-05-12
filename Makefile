build:
	cargo build --release
	rm -f ./lua/libroam_nvim.so
	cp ./target/release/libroam_nvim.so ./lua/libroam_nvim.so
	mkdir -p ./lua/deps
	cp ./target/release/deps/*.rlib ./lua/deps
