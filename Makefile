VERSION := $(shell cargo pkgid | awk -F'[@#]' '{print $$2}')

all: build install

build:
	cargo build --release --locked

install: target/release/nu_plugin_nutext tools/xnutext
	install -Dm755 target/release/nu_plugin_nutext -t $(DESTDIR)/usr/share/nutext/
	install -Dm755 tools/xnutext -t $(DESTDIR)/usr/bin/

uninstall:
	rm -fv /usr/share/nutext/nu_plugin_nutext /usr/bin/xnutext

dist: build tools/xnutext LICENSE README.md
	rm -rf dist/
	mkdir -p dist/
	tar -czvf dist/nutext-$(VERSION).tar.gz target/release/nu_plugin_nutext tools/xnutext LICENSE README.md
	sha256sum dist/nutext-$(VERSION).tar.gz > dist/SHA256SUM
