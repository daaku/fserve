pkgname=fserve
pkgver=2
pkgrel=1
pkgdesc='quick and lean static file serving'
arch=(x86_64)
url=https://github.com/daaku/$pkgname

package() {
  cd $srcdir/..
  go build
  install -d $pkgdir/usr/bin
  install -D -m 755 $srcdir/../$pkgname $pkgdir/usr/bin/$pkgname
}
