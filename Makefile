VERSION = 0.1.1

# commands
CMD_GET_RT = rustup target add
CMD_RELEASE = cargo build --release --target=

# rust targets
OSX_RT = x86_64-apple-darwin
WIN_RT = x86_64-pc-windows-msvc
LIN_RT = x86_64-unknown-linux-gnu
MSL_RT = x86_64-unknown-linux-musl

# targets
run-win:
	cls
	cargo run
run:
	clear
	cargo run
clean:
	cargo clean
list:
	rustup target list
toolchain:
	rustup update
	rustup component add clippy
	$(CMD_GET_RT) $(OSX_RT)
	$(CMD_GET_RT) $(WIN_RT)
	$(CMD_GET_RT) $(LIN_RT)
	$(CMD_GET_RT) $(MSL_RT)
lint:
	# cargo fix -Z unstable-options --clippy
	# cargo clippy
release: lint
	cargo build --release
release-all: lint
	$(CMD_RELEASE)$(OSX_RT)
	$(CMD_RELEASE)$(WIN_RT)
	$(CMD_RELEASE)$(LIN_RT)
	$(CMD_RELEASE)$(MSL_RT)
copy-all: copy-linux copy-osx copy-win
mk-release-dir:
	mkdir -p ./release
copy-linux: mk-release-dir
	tar -czvf ./release/inventory-linux-gnu-v$(VERSION).tar.gz ./target/$(LIN_RT)/release/inventory
	tar -czvf ./release/inventory-linux-musl-v$(VERSION).tar.gz ./target/$(MSL_RT)/release/inventory
copy-osx: mk-release-dir
	tar -czvf ./release/inventory-osx-v$(VERSION).tar.gz ./target/$(OSX_RT)/release/inventory
copy-win: mk-release-dir
	tar -czvf ./release/inventory-win-v$(VERSION).tar.gz ./target/$(OSX_RT)/release/inventory.exe