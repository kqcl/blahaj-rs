CARGO = cargo

TARGET = blahaj-rs

TARGET_DIR = target/release

SRC_DIR = src

build:
	$(CARGO) build --release

run:
	$(CARGO) run --release -- -s

clean:
	$(CARGO) clean 

default:
	build

.PHONY: build run clean defaul
