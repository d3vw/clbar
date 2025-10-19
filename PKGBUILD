# Maintainer: grey
pkgname=clbar
pkgver=0.1.0
pkgrel=1
pkgdesc="A lightweight system tray application for Hyprland to switch Clash proxy nodes"
arch=('x86_64')
url="https://github.com/yourusername/clbar"
license=('MIT')
depends=('libnotify')
makedepends=('cargo' 'git')
source=("git+$url.git#tag=v$pkgver")
sha256sums=('SKIP')

build() {
    cd "$pkgname"
    cargo build --release --locked
}

check() {
    cd "$pkgname"
    cargo test --release --locked
}

package() {
    cd "$pkgname"

    # Install binary
    install -Dm755 "target/release/$pkgname" "$pkgdir/usr/bin/$pkgname"

    # Install systemd service
    install -Dm644 "$pkgname.service" "$pkgdir/usr/lib/systemd/user/$pkgname.service"

    # Install license
    install -Dm644 LICENSE "$pkgdir/usr/share/licenses/$pkgname/LICENSE"

    # Install README
    install -Dm644 README.md "$pkgdir/usr/share/doc/$pkgname/README.md"
}
