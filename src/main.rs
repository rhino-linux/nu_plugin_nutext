use nu_plugin::{serve_plugin, MsgPackSerializer};
use nu_plugin_nutext::FluentPlugin;

fn main() {
    serve_plugin(&FluentPlugin, MsgPackSerializer)
}
