default: test

test:
	cargo test

fmt:
	cargo fmt

doc:
  cargo rustdoc --open -- --no-defaults \
        --passes collapse-docs   \
        --passes unindent-comments \
        --passes strip-priv-imports

run command='node':
	cargo run -- {{command}}

install-dev-deps-homebrew:
	brew install protobuf

install-dev-deps-apt:
	apt install protobuf-compiler
