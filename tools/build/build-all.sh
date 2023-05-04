#!/bin/sh
set -x -e
# Get version
VERSION="$1"
if [ -z "$VERSION" ]; then
    echo "VERSION is not set"
    exit 1
fi
# Create dist
[ -d dist/ ] || mkdir dist
# Define build function
build_target_tgz()
{
    cargo build --release --target $1
    (
        cd target/$1/release \
        && tar cfz gufo-agent.tgz gufo-agent \
        && mv gufo-agent.tgz ../../../dist/gufo-agent_XXX_$2.tgz
    )
}
# Build x86_64-unknown-linux-gnu
build_target_tgz x86_64-unknown-linux-gnu linux_amd64
