pkgname=fserve
pkgver=1
pkgrel=1
pkgdesc='quick and lean static file serving'
arch=(x86_64)
url=https://github.com/daaku/$pkgname

package() {
  install -d $pkgdir/usr/bin
  install -D -m 755 $srcdir/../target/release/$pkgname $pkgdir/usr/bin/$pkgname
}
