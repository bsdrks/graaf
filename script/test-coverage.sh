rm -rf ./target/coverage
rm -rf ./target/debug
rm -rf ./coverage
CARGO_INCREMENTAL=0 RUSTFLAGS='-Cinstrument-coverage' LLVM_PROFILE_FILE='coverage/cargo-test-%p-%m.profraw' cargo test --lib
grcov . --binary-path ./target/debug/deps/ -s . -t html --ignore-not-existing --ignore '../*' --ignore "/*" -o target/coverage/
grcov . --binary-path ./target/debug/deps/ -s . -t lcov --ignore-not-existing --ignore '../*' --ignore "/*" -o coverage/lcov.info