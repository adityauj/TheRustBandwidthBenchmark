TARGET = ./bench
VERSION = 1.0.0
GIT_HASH := $(shell git rev-parse --short HEAD || echo 'development')
CURRENT_TIME = $(shell date +"%Y-%m-%d:T%H:%M:%S")
LD_FLAGS = '-s -X main.date=${CURRENT_TIME} -X main.version=${VERSION} -X main.commit=${GIT_HASH}'

.PHONY: clean test check install asm $(TARGET)

.NOTPARALLEL:

$(TARGET):
	$(info ===>  BUILD bench)
	@cargo b --release
	@cp ./target/release/$(TARGET) $(TARGET)

install:
	$(info ===>  INSTALL)
	@cargo install cargo-asm

asm:
	$(info ===>  ASM)
	@cargo rustc --release -- --emit asm

clean:
	$(info ===>  CLEAN)
	@cargo clean
	@rm -f $(TARGET)
	@rm -f *.data *svg *.data.old

test:
	$(info ===>  TESTING)
	@cargo clean
	@cargo b --release
	@cargo test

check:
	$(info ===>  CHECKING)
	@cargo clean
	@cargo check