default:
	@just --list

# all
all: build run

# build
build:
	cargo build

# run
run:
	cargo run

# test
test:
	. ./utils/get_shapes.sh

# clean
clean:
	rm -rf ./target
