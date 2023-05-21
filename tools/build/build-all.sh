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
build_deb()
{
    arch=$1
    root="dist/deb/$arch"
    # Cleanup and recreate directory
    [ -d $root ] && rm -r $root
    mkdir -p $root
    # Inner directory structure
    mkdir -p "$root/DEBIAN"
    mkdir -p "$root/usr/bin"
    mkdir -p "$root/usr/share/man/man1"
    # Copy binary
    tar -x -z -f ./dist/gufo-agent-${VERSION}_linux_$arch.tgz -C $root/usr/bin
    # Copy man
    cp man/gufo-agent.1 $root/usr/share/man/man1
    # Get package size
    PACKAGE_ROOT_SIZE_BYTES=$(du -bcs --exclude=DEBIAN $root | head -1 | awk '{print $1}' | sed -e s/^0\+//)
    # To kB
    INPUT_INSTALLED_SIZE="$(awk -v size="$PACKAGE_ROOT_SIZE_BYTES" 'BEGIN {print (size/1024)+1}' | awk '{print int($0)}')"
    # Write DEBIAN/control
    cat > $root/DEBIAN/control << __EOF__
Package: gufo-agent
Version: ${VERSION}
Installed-Size: ${INPUT_INSTALLED_SIZE}
Architecture: ${arch}
Maintainer: Gufo Labs

an universal agent for infrastructure monitoring
__EOF__
    # Build deb
    dpkg-deb -Zgzip -b $root "dist/gufo-agent_${VERSION}_${arch}.deb"
}
# Prepare structure for rpm package
# $1 - arch
prepare_rpm()
{
    root="dist/rpm/$1"
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
build_deb amd64
prepare_rpm amd64
# Build aarch64-unknown-linux-gnu + deb
build_target_tgz aarch64-unknown-linux-gnu linux_aarch64
build_deb aarch64
prepare_rpm aarch64
