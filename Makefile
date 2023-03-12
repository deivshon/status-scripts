TARGET_DIR = ./target/release

all:
	cargo build --release

clean:
	cargo clean

install: all
	mkdir -p ~/.local/scripts
	cp -f $(TARGET_DIR)/*-status ~/.local/scripts
