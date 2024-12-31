all: build install

build:
	cargo build --release --locked

install: target/release/nu_plugin_nutext
	install -Dm755 target/release/nu_plugin_nutext -t $(DESTDIR)/usr/share/nutext/
