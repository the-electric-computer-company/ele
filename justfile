# default to `test`
default: test

log='warn'

bt='0'

version = `head -3 Cargo.toml | sed -En 's/^version[[:space:]]*=[[:space:]]*"([^"]+)"/v\1/p'`

export RUST_BACKTRACE = bt

# run tests
test:
	cargo test

# format source with rustfmt
fmt:
	cargo fmt

# run linter
@lint:
	echo Checking for TODO/FIX/XXX...
	! grep --color -En 'TODO|FIX|XXX' src/*.rs
	echo Checking for lines over 100 columns...
	! grep --color -En '.{101}' src/*.rs
	echo Invoking clippy...
	cargo +nightly clippy -- \
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

# assert that the repo has no staged or unstaged changes
assert-clean:
	git diff --no-ext-diff --quiet --exit-code

# push the current branch to upstream
pr: fmt lint test assert-clean
	[ `git rev-parse --abbrev-ref HEAD` != master ]
	git push upstream

# run a command, defaulting to `node`
run command='node': build
	RUST_LOG={{log}} ./target/debug/ele {{command}}

# install development dependencies
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

# To tag and publish a release:
#
# - PR and merge all changes for release
# - `git co master && git pull --rebase upstream master`
# - `git branch -b release-{{version}}`
# - `just tag`
# - Open PR with `release-{{version}}` branch, wait for tests to pass, and merge
# - Wait for release binaries to appear on github (this process is kicked off by `just tag`)
# - `git co master && git pull --rebase upstream master`
#	- `just publish`

# create a release tag and push it to github
tag: fmt lint test assert-clean
	[ `git rev-parse --abbrev-ref HEAD` == release-{{version}} ]
	git diff --no-ext-diff --quiet --exit-code master
	cargo test --release
	git tag -a {{version}} -m 'Release {{version}}'
	git push upstream release-{{version}}
	git push upstream {{version}}

# publish a release to crates.io
publish: fmt lint test assert-clean
	[ `git rev-parse --abbrev-ref HEAD` == master ]
	git fetch
	git checkout {{version}}
	cargo publish

