all: build install

build:
	cargo build --release --locked

install: target/release/nu_plugin_nutext tools/xnutext
	install -Dm755 target/release/nu_plugin_nutext -t $(DESTDIR)/usr/share/nutext/
	install -Dm755 tools/xnutext -t $(DESTDIR)/usr/bin/
