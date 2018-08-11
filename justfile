default: test

test:
	cargo test

fmt:
	cargo fmt

doc:
	cargo doc --all --open

install-dev-deps-homebrew:
	brew install protobuf

install-dev-deps-apt:
	apt install protobuf-compiler
