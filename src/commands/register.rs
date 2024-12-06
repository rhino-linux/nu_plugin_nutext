use nu_plugin::{EngineInterface, EvaluatedCall, SimplePluginCommand};
use nu_protocol::{LabeledError, Signature, SyntaxShape, Value};

use crate::FluentPlugin;

pub struct Register;

impl SimplePluginCommand for Register {
    type Plugin = FluentPlugin;

    fn name(&self) -> &str {
        "tregister"
    }

    fn description(&self) -> &str {
        "Register locales."
    }

    fn signature(&self) -> Signature {
        Signature::build(self.name()).required(
            "dir",
            SyntaxShape::Directory,
            "directory containing locales",
        )
    }

    fn run(
        &self,
        _plugin: &Self::Plugin,
        _engine: &EngineInterface,
        call: &EvaluatedCall,
        _input: &Value,
    ) -> Result<Value, LabeledError> {
        let strang: String = call.req(0).unwrap();
        let dir = format!("{strang}/{{locale}}");
        println!("{dir}");
        Ok(Value::nothing(call.head))
    }
}
