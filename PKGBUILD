# Maintainer: Michael Hübner <michaelh.95@t-online.de>

pkgname=pulsesink
pkgver=0.1.1
pkgrel=1
pkgdesc="GUI manager for PulseAudio sinks"
arch=('i686' 'x86_64')
url="https://github.com/hodasemi/pulsesink"
license=('MIT')
depends=('gtk3' 'pulseaudio')
makedepends=('cargo')
source=("$url/archive/$pkgver.tar.gz")
sha256sums=('1c9fba30f5b03b66851b61f69e321399512c9eacb2697525910877d2b3449207')

build() {
	cd "$pkgname-$pkgver"
	cargo build --release
}

package () {
	cd "$pkgname-$pkgver"
	install -Dm 755 target/release/pulsesink -t "$pkgdir/usr/bin/"
	install -Dm 644 pulsesink.desktop -t "$pkgdir/usr/share/applications"
}
