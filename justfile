default: test

test:
	cargo test

fmt:
	cargo fmt

doc:
	cargo rustdoc --open -- --document-private-items

run command='node':
	cargo run -- {{command}}

install-dev-deps-homebrew:
	brew install protobuf

install-dev-deps-apt:
	apt install protobuf-compiler
