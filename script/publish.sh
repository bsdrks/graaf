#!/bin/sh

# Version check

# Version check: Cargo.toml

CARGO_TOML_VERSION_REGEX='name = "graaf"
version = "([0-9]*\.[0-9]*\.[0-9]*)"'

cargo_toml=$(cat Cargo.toml)

if [[ $cargo_toml =~ $CARGO_TOML_VERSION_REGEX ]]; then
    # This is our source of truth for the crate version. 
    VERSION=${BASH_REMATCH[1]}
else
    echo "Error: version not found in Cargo.toml"
    exit 1
fi

echo "✓ Version in \`Cargo.toml\`: $VERSION"

# Version check: README.md

DEPENDENCY_SIMPLE_VERSION_REGEX='graaf = "([0-9]*\.[0-9]*\.[0-9]*)"'
DEPENDENCY_OBJECT_VERSION_REGEX='graaf = { version = "([0-9]*\.[0-9]*\.[0-9]*)"'

readme=$(cat README.md)

while IFS= read -r line || [[ -n $line ]]; do
    if [[ $line =~ $DEPENDENCY_SIMPLE_VERSION_REGEX ]]; then
        DEPENDENCY_VERSION=${BASH_REMATCH[1]}

        if [[ $DEPENDENCY_VERSION != $VERSION ]]; then
            echo "Error: expected dependency example version $VERSION, found $DEPENDENCY_VERSION"
            exit 1
        fi
    elif [[ $line =~ $DEPENDENCY_OBJECT_VERSION_REGEX ]]; then
        DEPENDENCY_VERSION=${BASH_REMATCH[1]}

        if [[ $DEPENDENCY_VERSION != $VERSION ]]; then
            echo "Error: expected dependency example version $VERSION, found $DEPENDENCY_VERSION"
            exit 1
        fi
    fi
done <<< "$readme"

echo "✓ Version in \`README.md\`: $VERSION"

# Version check: CHANGELOG.md

CHANGELOG_VERSION_REGEX="## \[([0-9]*\.[0-9]*\.[0-9]*)\] - [0-9]{4}-[0-9]{2}-[0-9]{2}"

changelog=$(cat CHANGELOG.md)

if [[ $changelog =~ $CHANGELOG_VERSION_REGEX ]]; then
    CHANGELOG_VERSION=${BASH_REMATCH[1]}

    if [[ $CHANGELOG_VERSION != $VERSION ]]; then
        echo "Error: expected top changelog entry to be $VERSION, found $CHANGELOG_VERSION"
        exit 1
    fi
else
    echo "Error: no changelog entry found."
    exit 1
fi

echo "︎✓ Version in latest changelog entry: $VERSION"

# Static analysis

cargo fmt --check --all
cargo doc --all-features
cargo clippy --all-targets

# Test

CARGO_INCREMENTAL=0 RUSTFLAGS='-Cinstrument-coverage' LLVM_PROFILE_FILE='coverage/cargo-test-%p-%m.profraw' cargo test

# Test coverage

total_coverage=$(grcov . --binary-path ./target/debug/ -t markdown -s . --ignore '../*' --ignore '/*' | tail -n 1)

if [[ "$total_coverage" != *'100.00%'* ]]; then
    echo 'Coverage is not 100.00%'
    exit 1
else
    echo '✓ Coverage is 100.00%'
fi

# Publish

cargo publish