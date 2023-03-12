TARGET_DIR = ./target/release

all:
	cargo build --release

clean:
	cargo clean

install: all
	mkdir -p ~/.local/scripts
	cp $(TARGET_DIR)/*-status ~/.local/scripts
