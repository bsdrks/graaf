set -e

if ! command -v grcov &> /dev/null; then
    echo "Error: grcov is not installed. Please install it with:"
    echo "  cargo install grcov"
    exit 1
fi

# Check for llvm-profdata in the rustup toolchain directory
RUSTC_SYSROOT=$(rustc --print sysroot 2>/dev/null || echo "")
if [ -z "$RUSTC_SYSROOT" ]; then
    echo "Error: rustc is not available"
    exit 1
fi

RUSTC_HOST=$(rustc -vV 2>/dev/null | sed -n 's|host: ||p')
LLVM_PROFDATA_PATH="$RUSTC_SYSROOT/lib/rustlib/$RUSTC_HOST/bin/llvm-profdata"

if [ ! -f "$LLVM_PROFDATA_PATH" ]; then
    echo "Error: llvm-profdata is not installed. Please install it with:"
    echo "  rustup component add llvm-tools"
    exit 1
fi

rm -rf ./target/coverage
rm -rf ./target/debug
rm -rf ./coverage

CARGO_INCREMENTAL=0 RUSTFLAGS='-Cinstrument-coverage' LLVM_PROFILE_FILE='coverage/cargo-test-%p-%m.profraw' cargo test --lib

grcov . --binary-path ./target/debug/deps/ -s . -t html --ignore-not-existing --ignore '../*' --ignore "/*" -o target/coverage/
grcov . --binary-path ./target/debug/deps/ -s . --ignore-not-existing --ignore '../*' --ignore "/*" -o target/coverage/

total_coverage=$(grcov . --binary-path ./target/debug/ -t markdown -s . --ignore '../*' --ignore '/*' | tail -n 1)

if [[ "$total_coverage" != *'100.00%'* ]]; then
    echo "Coverage is not 100.00%: $total_coverage"
    exit 1
else
    echo '✓ Coverage is 100.00%'
fi