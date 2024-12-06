use fluent_fallback::Localization;
use fluent_resmgr::ResourceManager;
use nu_plugin::{Plugin, PluginCommand};
use std::sync::Mutex;

mod commands;

use crate::commands::print::Print;
use crate::commands::register::Register;

pub struct FluentPlugin {
    msgr: Mutex<Localization<Vec<&'static str>, ResourceManager>>,
}

impl Plugin for FluentPlugin {
    fn version(&self) -> String {
        env!("CARGO_PKG_VERSION").into()
    }

    fn commands(&self) -> Vec<Box<dyn PluginCommand<Plugin = Self>>> {
        vec![Box::new(Print), Box::new(Register)]
    }
}
