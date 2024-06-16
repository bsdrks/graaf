#!/bin/sh

# Check the versions.

## Check the version in `Cargo.toml`.

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

## Check the version in the `README.md` example.

DEPENDENCY_SIMPLE_VERSION_REGEX='graaf = "([0-9]*\.[0-9]*\.[0-9]*)"'

readme=$(cat README.md)

while IFS= read -r line || [[ -n $line ]]; do
    if [[ $line =~ $DEPENDENCY_SIMPLE_VERSION_REGEX ]]; then
        DEPENDENCY_VERSION=${BASH_REMATCH[1]}

        if [[ $DEPENDENCY_VERSION != $VERSION ]]; then
            echo "Error: expected dependency example version $VERSION, found $DEPENDENCY_VERSION"
            exit 1
        fi
    fi
done <<< "$readme"

echo "✓ Version in \`README.md\`: $VERSION"

## Check version in latest the `CHANGELOG.md` entry.

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

# Run static analysis

cargo fmt --check --all
cargo doc --all-features
cargo clippy --all-targets

# Publish

cargo publish