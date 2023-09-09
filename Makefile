TARGET_DIR = ./target/release
INSTALL_DIR=~/.local/bin

all:
	cargo build --release

clean:
	cargo clean

install: all
	mkdir -p $(INSTALL_DIR)
	cp -f $(TARGET_DIR)/*-status $(INSTALL_DIR)
	cp -f $(TARGET_DIR)/battery-notifier $(INSTALL_DIR)
