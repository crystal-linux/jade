# Maintainer: Matt C <mdc028[at]bucknell[dot]edu>

pkgname=jade
pkgver=1.0.5
pkgrel=1
pkgdesc="Scriptable backend & TUI Installer for Crystal Linux"
license=('GPL3')
arch=('x86_64')
url="https://git.tar.black/crystal/programs/jade"
license=('Nolicense')
source=("git+https://git.tar.black/crystal/programs/jade")
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
