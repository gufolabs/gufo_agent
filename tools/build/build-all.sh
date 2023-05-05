#!/bin/sh
set -x -e
# Get version
VERSION="$1"
if [ -z "$VERSION" ]; then
    echo "VERSION is not set"
    exit 1
fi
echo "Building gufo-agent ${VERSION}"
# Create dist
[ -d dist/ ] || mkdir dist
# Define build function
# $1 - rust target
# $2 - distribution platform name
build_target_tgz()
{
    cross build --release --target $1
    (
        cd target/$1/release \
        && tar cfz gufo-agent.tgz gufo-agent \
        && mv gufo-agent.tgz ../../../dist/gufo-agent-${VERSION}_$2.tgz
    )
}
# Build x86_64-unknown-linux-gnu
build_target_tgz x86_64-unknown-linux-gnu:centos linux_amd64
