#!/bin/sh
# This is a little script which can be downloaded from internet
# to install Gufo Agent. It just performs platform detection,
# downloads, and runs installer.
set -e

VERSION="0.9.0a2"

# Download file using curl
# $1 url
# $2 path
download_curl() {
    curl -q -o $2 $1
}

# Download file using wget
# $1 url
# $2 path
download_wget() {
    wget -O $2 $1
}

# Get available downloader
# Retunrs:
# * curl
# * wget
# * other
get_downloader() {
    r=$(which curl)
    if [ ! -z "$r" ]; then
        echo "curl"
        return
    fi
    r=$(which wget)
    if [ ! -z "$r" ]; then
        echo "wget"
        return
    fi
    echo "other"
}

# Download file
# $1 url
# $2 path
download() {
    case "$(get_downloader)" in
        curl)
            download_curl $1 $2
            ;;
        wget)
            download_wget $1 $2
            ;;
        *)
            die "Cannot find curl or wget"
            ;;
    esac
}

# Download github artefact for latest release
# $1 - artefact name
# Output - file name
get_release_artefact() {
    url="https://github.com/gufolabs/gufo_agent/releases/download/v${VERSION}/$1"
    out_dir=$(mktemp -d)
    out_path="${out_dir}/$1"
    download $url $out_path
    echo $out_path
}

# Debian installation
install_linux_debian() {
    arch=$(uname -m)
    case "$arch" in
        x86_64)
            art_name=gufo-agent-${VERSION}_amd64.deb
            ;;
        *)
            die "Unsupported debian arch: $arch"
            ;;
    esac
    # Download .deb
    path=$(get_release_artefact $art_name)
    # Install .deb
    dpkg -i $path
    # Cleanup
    rm -r $(dirname $path)
}

# Detect linux kind
# Retuns:
# * debian
# * other
get_linux_kind() {
    if [ -f "/etc/debian_version" ]; then
        echo "debian"
        return
    fi
    echo "other"
}

# Linux installation
install_linux() {
    kind=$(get_linux_kind)
    case "$kind" in
        debian)
            install_linux_debian
            ;;
        *)
            die "Unsupported linux $kind"
    esac
}

# Print message and exit with error
die() {
    echo "$1"
    exit 1
}

# Entrypoint
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

main "$@" || exit 1