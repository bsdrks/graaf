#!/bin/sh

set -e

# Validate the arguments.

VERSION_REGEX='([0-9]*\.[0-9]*\.[0-9]*)'

if [[ $1 =~ $VERSION_REGEX ]]; then
    VERSION_OLD=${BASH_REMATCH[1]}
else
    echo "Error: invalid version $1"
    exit 1
fi

if [[ $2 =~ $VERSION_REGEX ]]; then
    VERSION_NEW=${BASH_REMATCH[1]}
else
    echo "Error: invalid version $2"
    exit 1
fi

## Check the version in `Cargo.toml`.

CARGO_TOML_VERSION_REGEX='name = "graaf"
version = "([0-9]*\.[0-9]*\.[0-9]*)"'

cargo_toml=$(cat Cargo.toml)

if [[ $cargo_toml =~ $VERSION_REGEX ]]; then
  if [[ ${BASH_REMATCH[1]} != $VERSION_OLD ]]; then
    echo "Error: expected version $VERSION_OLD in Cargo.toml, found ${BASH_REMATCH[1]}"
    exit 1
  fi
else
    echo "Error: version not found in Cargo.toml"
    exit 1
fi

## Check the version in the `README.md` example.

DEPENDENCY_SIMPLE_VERSION_REGEX='graaf = "([0-9]*\.[0-9]*\.[0-9]*)"'

readme=$(cat README.md)

if [[ $readme =~ $DEPENDENCY_SIMPLE_VERSION_REGEX ]]; then
    DEPENDENCY_VERSION=${BASH_REMATCH[1]}

    if [[ $DEPENDENCY_VERSION != $VERSION_OLD ]]; then
        echo "Error: expected dependency example version $VERSION_OLD, found $DEPENDENCY_VERSION"
        exit 1
    fi
else
    echo "Error: dependency example not found in README.md"
    exit 1
fi

## Check the version in the latest `CHANGELOG.md` entry.

CHANGELOG_VERSION_REGEX="## \[([0-9]*\.[0-9]*\.[0-9]*)\] - [0-9]{4}-[0-9]{2}-[0-9]{2}"

changelog=$(cat CHANGELOG.md)

if [[ $changelog =~ $CHANGELOG_VERSION_REGEX ]]; then
    CHANGELOG_VERSION=${BASH_REMATCH[1]}

    if [[ $CHANGELOG_VERSION != $VERSION_NEW ]]; then
        echo "Error: expected top changelog entry to be $VERSION_NEW, found $CHANGELOG_VERSION"
        exit 1
    fi
else
    echo "Error: no changelog entry found."
    exit 1
fi

# Update the versions.

## Update the version in `Cargo.toml`.

sed -i '' "s/version = \"$VERSION_OLD\"/version = \"$VERSION_NEW\"/" Cargo.toml

## Update the version in the `README.md` example.

sed -i '' "s/graaf = \"$VERSION_OLD\"/graaf = \"$VERSION_NEW\"/" README.md