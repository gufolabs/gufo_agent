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
    # Build
    cross build --release --target $1
    # Copy binary distribution
    (
        cd target/$1/release \
        && tar cfz gufo-agent.tgz gufo-agent \
        && mv gufo-agent.tgz ../../../dist/gufo-agent-${VERSION}_$2.tgz
    )
}
# Prepare structure for debian package
# $1 - arch
prepare_deb()
{
    root="dist/deb/$1"
    # Cleanup and recreate directory
    [ -d $root ] && rm -r $root
    mkdir -p $root
    # Inner directory structure
    mkdir -p "$root/usr/bin"
    mkdir -p "$root/usr/share/man/man1"
    # Copy binary
    tar -x -z -f ./dist/gufo-agent-${VERSION}_linux_$1.tgz -C $root/usr/bin
    # Copy man
    cp man/gufo-agent.1 $root/usr/share/man/man1
}
# Build x86_64-unknown-linux-gnu + deb
build_target_tgz x86_64-unknown-linux-gnu linux_amd64
prepare_deb amd64
# Build aarch64-unknown-linux-gnu + deb
build_target_tgz aarch64-unknown-linux-gnu linux_aarch64
prepare_deb aarch64