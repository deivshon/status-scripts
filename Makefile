CC = gcc

UTILS_PATH = .
COMPONENTS_PATH = ./components

CFLAGS = -O2 -Wall -Wextra

all: eth-status disk-status ram-status cpu-status mullvad-check-status battery-status wifi-status

%-status: $(COMPONENTS_PATH)/%.o $(UTILS_PATH)/utils.o
	$(CC) $(CFLAGS) -o $@ $^

%.o: %.c $(UTILS_PATH)/utils.c $(UTILS_PATH)/utils.h
	$(CC) $(CFLAGS) -c -o $@ $<

clean:
	rm -f *-status */*.o $(UTILS_PATH)/*.o

install: all
	mkdir -p ~/.local/scripts
	cp *-status ~/.local/scripts
