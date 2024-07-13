set -e

rm -rf ./target/coverage
rm -rf ./target/debug
rm -rf ./coverage

CARGO_INCREMENTAL=0 RUSTFLAGS='-Cinstrument-coverage' LLVM_PROFILE_FILE='coverage/cargo-test-%p-%m.profraw' cargo test --lib

grcov . --binary-path ./target/debug/deps/ -s . -t html --ignore-not-existing --ignore '../*' --ignore "/*" -o target/coverage/

total_coverage=$(grcov . --binary-path ./target/debug/ -t markdown -s . --ignore '../*' --ignore '/*' | tail -n 1)

if [[ "$total_coverage" != *'100.00%'* ]]; then
    echo "Coverage is not 100.00%: $total_coverage"
    exit 1
else
    echo '✓ Coverage is 100.00%'
fi