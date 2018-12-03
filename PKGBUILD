# Maintainer: Michael Hübner <michaelh.95@t-online.de>

pkgname=pulsesink
pkgver=v0.1
pkgrel=1
pkgdesc="GUI manager for PulseAudio sinks"
arch=('i686' 'x86_64')
url="https://github.com/hodasemi/pulsesink"
license=('MIT')
depends=('gtk3' 'pulseaudio')
makedepends=('cargo')
source=("$url/archive/$pkgver.tar.gz")

build() {
	cd "$pkgname-$pkgver"
	cargo build --release
}

package () {
	cd "$pkgname-$pkgver"
	install -Dm 755 target/release/pulsesink -t "$pkgdir/usr/bin/pulsesink"
	install -Dm 644 pulsesink.desktop -t "$pkgdir/usr/share/applications"
	install -Dm 644 pulsesink.glade -t "$pkgdir/usr/bin/pulsesink"
}
