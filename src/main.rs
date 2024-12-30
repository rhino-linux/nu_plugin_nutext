use nu_plugin::{serve_plugin, MsgPackSerializer};
use nu_plugin_nutext::PrintPlugin;

fn main() {
    serve_plugin(&PrintPlugin, MsgPackSerializer);
}
