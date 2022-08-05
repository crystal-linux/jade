# Maintainer: Matt C <matt[at]getcryst[dot]al>

pkgname=jade
pkgver=1.0.7
pkgrel=1
pkgdesc="Scriptable backend & TUI Installer for Crystal Linux"
license=('GPL3')
arch=('x86_64')
url="https://github.com/crystal-linux/jade"
license=('Nolicense')
source=("git+$url")
sha256sums=('SKIP')
depends=('parted')
makedepends=('cargo')

build() {
    cd ${srcdir}/jade
    cargo build --release
}

package() {
    mkdir -p $pkgdir/usr/bin
    chmod +x ${srcdir}/jade/target/release/jade
    cp ${srcdir}/jade/target/release/jade  $pkgdir/usr/bin/.
}
