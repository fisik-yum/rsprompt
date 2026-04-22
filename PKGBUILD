# Maintainer: github.com/fisik-yum

_pkgbase=rsprompt
pkgname=${_pkgbase}-git
pkgver=0.1
pkgrel=0
pkgdesc="bash prompt generator"
url="https://github.com/fisik-yum/rsprompt"
license=('GPL')
arch=('x86_64')
depends=('cargo')
makedepends=('git')
conflicts=("${_pkgbase}")
provides=("${_pkgbase}")
sha256sums=('SKIP')

source=("$_pkgbase"::"git+$url.git")

build() {
    cd "$_pkgbase"
    cargo build --release
}

package() {
    cd "$_pkgbase"
    install -Dm755 target/release/rsprompt "$pkgdir/usr/bin/rsprompt"
}
