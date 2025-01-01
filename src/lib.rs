use nu_plugin::{Plugin, PluginCommand};

mod commands;

use crate::commands::print::Print;
use crate::commands::register::Register;
use crate::commands::stringret::StringRet;

pub struct PrintPlugin;

impl Plugin for PrintPlugin {
    fn version(&self) -> String {
        env!("CARGO_PKG_VERSION").into()
    }

    fn commands(&self) -> Vec<Box<dyn PluginCommand<Plugin = Self>>> {
        vec![Box::new(Print), Box::new(Register), Box::new(StringRet)]
    }
}
