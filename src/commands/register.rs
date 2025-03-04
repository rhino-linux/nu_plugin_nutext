use std::path::PathBuf;

use nu_plugin::{EngineInterface, EvaluatedCall, SimplePluginCommand};
use nu_protocol::{record, LabeledError, Signature, Span, SyntaxShape, Value};

use crate::PrintPlugin;

pub struct Register;

impl SimplePluginCommand for Register {
    type Plugin = PrintPlugin;

    fn name(&self) -> &'static str {
        "tregister"
    }

    fn description(&self) -> &'static str {
        "Register locales."
    }

    fn signature(&self) -> Signature {
        Signature::build(self.name())
            .required("dir", SyntaxShape::Filepath, "directory containing locales")
            .required("name", SyntaxShape::String, "program name")
    }

    fn run(
        &self,
        _plugin: &Self::Plugin,
        engine: &EngineInterface,
        call: &EvaluatedCall,
        _input: &Value,
    ) -> Result<Value, LabeledError> {
        let path: PathBuf = call.req(0).unwrap();
        let name: String = call.req(1).unwrap();

        let validated_path = if path.exists() {
            path
        } else {
            return Err(LabeledError::new("Cannot find path.")
                .with_help(format!("Verify that `{}` exists.", path.display())));
        };

        engine
            .add_env_var(
                "NUTEXT_FILES",
                Value::Record {
                    val: record! {
                        "path" => Value::string(validated_path.to_str().unwrap(), Span::unknown()),
                        "name" => Value::string(name, Span::unknown())
                    }
                    .into(),
                    internal_span: Span::unknown(),
                },
            )
            .unwrap();
        Ok(Value::nothing(call.head))
    }
}
