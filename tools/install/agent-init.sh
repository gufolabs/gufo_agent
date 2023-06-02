#!/bin/sh
# This is a little script which can be downloaded from internet
# to install Gufo Agent. It just performs platform detection,
# downloads, and runs installer.
set -e

VERSION="0.9.0a7"

main() {
    os=$(uname)
    case "$os" in
        Linux)
            install_linux
            ;;
        *)
            die "Unsupported OS: $os"
            ;;
    esac
}

die() {
    echo "$1"
    exit 1
}

# Install on Linux
install_linux() {
    get_linux_kind
    case "$RETVAL" in
        debian)
            install_linux_debian
            ;;
        *)
            die "Unsupported linux $kind"
    esac
}

# Detect linux kind
# $RETVAL:
# * debian
# * other
get_linux_kind() {
    if [ -f "/etc/debian_version" ]; then
        RETVAL=debian
        return
    fi
    RETVAL=other
}

# Debian installation
install_linux_debian() {
    arch=$(uname -m)
    case "$arch" in
        x86_64)
            art_name=gufo-agent_${VERSION}_amd64.deb
            ;;
        aarch64)
            art_name=gufo-agent_${VERSION}_aarch64.deb
            ;;
        *)
            die "Unsupported debian arch: $arch"
            ;;
    esac
    # Download .deb
    get_release_artefact $art_name
    path=$RETVAL
    # Install .deb
    dpkg -i $path
    # Cleanup
    rm -r $(dirname $path)
}

# Download github artefact for latest release
# $1 - artefact name
# $RETVAL: file name
get_release_artefact() {
    url="https://github.com/gufolabs/gufo_agent/releases/download/v${VERSION}/$1"
    out_dir=$(mktemp -d)
    out_path="${out_dir}/$1"
    download $url $out_path
    RETVAL=$out_path
}

# Download file
# $1 url
# $2 path
download() {
    get_downloader
    case $RETVAL in
        curl)
            download_curl $1 $2
            ;;
        wget)
            download_wget $1 $2
            ;;
    esac
}

# Get available downloader
# $RETVAL:
# * curl
# * wget
get_downloader() {
    r=$(which curl)
    if [ ! -z "$r" ]; then
        RETVAL=curl
        return
    fi
    r=$(which wget)
    if [ ! -z "$r" ]; then
        RETVAL=wget
        return
    fi
    die "Cannot find downloader"
}

# Download file using curl
# $1 url
# $2 path
download_curl() {
    echo $1
    curl -L -f -s -o $2 $1 || die "curl: Failed to download $1"
}

# Download file using wget
# $1 url
# $2 path
download_wget() {
    wget -O $2 $1  || die "curl: Failed to download $1"
}

main "$@" || exit 1