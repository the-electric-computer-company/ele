# This script takes care of testing your crate

set -ex

# DONE This is the "test phase", tweak it as you see fit
main() {
    cross build --target $TARGET

    if [ ! -z $DISABLE_TESTS ]; then
        return
    fi

    cross fmt -- --check

    cross test --target $TARGET

    cross run --target $TARGET -- --version
}

# we don't run the "test phase" when doing deploys
if [ -z $TRAVIS_TAG ]; then
    main
fi
