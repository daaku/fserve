importpath=github.com/daaku/fserve
pkgname=$(basename "$importpath")
pkgver=$(git rev-list --count HEAD)
pkgrel=1
pkgdesc='quick and lean static file serving'
arch=('x86_64' 'aarch64')
url="https://$importpath"
license=('MIT')
makedepends=('go')

build() {
  cd ..
  export CGO_ENABLED=0
  go build -trimpath -o fserve
}

package() {
  cd ..
  install -Dm755 fserve "$pkgdir/usr/bin/fserve"
  install -Dm644 license "$pkgdir/usr/share/licenses/$pkgname/LICENSE"
}
