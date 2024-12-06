use nu_plugin::{EngineInterface, EvaluatedCall, SimplePluginCommand};
use nu_protocol::{LabeledError, Signature, SyntaxShape, Value};

use crate::FluentPlugin;

pub struct Print;

impl SimplePluginCommand for Print {
    type Plugin = FluentPlugin;

    fn name(&self) -> &str {
        "tprint"
    }

    fn description(&self) -> &str {
        "Print out translated strings."
    }

    fn signature(&self) -> Signature {
        Signature::build(self.name())
            .switch("stderr", "print to stderr instead of stdout", Some('e'))
            .switch(
                "no-newline",
                "print without inserting a newline for the line ending",
                Some('n'),
            )
            .required("key", SyntaxShape::String, "fluent key")
            .rest("rest", SyntaxShape::String, "interpolation variables")
    }

    fn run(
        &self,
        _plugin: &Self::Plugin,
        _engine: &EngineInterface,
        call: &EvaluatedCall,
        _input: &Value,
    ) -> Result<Value, LabeledError> {
        // Pretty sure this is already checked in [`signature`].
        let to_print: String = call.req(0).unwrap();
        match call.has_flag("stderr") {
            Ok(b) => {
                if b {
                    eprintln!("{to_print}")
                } else {
                    println!("{to_print}")
                }
            }
            Err(_) => eprintln!("{to_print}"),
        }
        Ok(Value::nothing(call.head))
    }
}
