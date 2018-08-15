# default to `test`
default: test

log='warn'

# run tests
test:
	cargo test

# format source with rustfmt
fmt:
	cargo fmt

# run linter
lint:
	@echo Running clippy...
	@cargo +nightly clippy -- \
		-D clippy \
		-D clippy_style \
		-D clippy_complexity \
		-D clippy_correctness \
		-D clippy-perf

# build and open docs
doc:
	cargo rustdoc --open -- --document-private-items

# watch for changes and run `cargo fmt` and `cargo check`
watch:
	cargo watch --ignore 'src/svc/*' --clear --exec test

# build
build:
	cargo build

# check
check:
	cargo check

# count non-empty lines of code
sloc:
	@cat src/*.rs | sed '/^\s*$/d' | wc -l

# run a command, defaulting to `node`
run command='node': build
	RUST_LOG={{log}} ./target/debug/ele {{command}}

install-dev-deps:
	# for `lint` recipe
	rustup component add clippy-preview --toolchain=nightly
	# for `fmt` recipe
	rustup component add rustfmt-preview
	# for `watch` recipe
	cargo install cargo-watch 

# install development dependencies using homebrew
install-dev-deps-homebrew:
	brew install protobuf

# install development dependencies using apt
install-dev-deps-apt:
	apt install protobuf-compiler

# configure git to skip diffing generated code
configure-nodiff-driver:
	git config diff.nodiff.command true
