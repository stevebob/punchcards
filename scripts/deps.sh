if [ -z ${TRAVIS_OS_NAME+x} ]; then
    case `uname -s` in
        Linux)
            TRAVIS_OS_NAME=linux
            ;;
        Darwin)
            TRAVIS_OS_NAME=osx
            ;;
        *)
            echo "Unknown OS"
            exit 1
    esac
fi

echo ">>>>> RUST_VERSION: $TRAVIS_RUST_VERSION"

case $TRAVIS_OS_NAME in
    linux)
        pyenv version 3.6
        PIP=pip
        PYTHON=python

        rustup target add wasm32-unknown-unknown
        cargo install --git https://github.com/alexcrichton/wasm-gc
        ;;
    osx)
        PIP=pip3
        PYTHON=python3
        ;;
    local-archlinux)
        PIP=pip3.6
        PYTHON=python3.6
        ;;
esac

$PIP install --quiet --user sh toml
