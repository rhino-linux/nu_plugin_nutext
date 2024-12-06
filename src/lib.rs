use fluent_resmgr::ResourceManager;
use nu_plugin::{Plugin, PluginCommand};

mod commands;

use crate::commands::print::Print;
use crate::commands::register::Register;

pub struct FluentPlugin;

impl Plugin for FluentPlugin {
    fn version(&self) -> String {
        env!("CARGO_PKG_VERSION").into()
    }

    fn commands(&self) -> Vec<Box<dyn PluginCommand<Plugin = Self>>> {
        vec![Box::new(Print), Box::new(Register)]
    }
}
