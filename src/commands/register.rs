use nu_plugin::{EngineInterface, EvaluatedCall, SimplePluginCommand};
use nu_protocol::{LabeledError, Record, Signature, Span, SyntaxShape, Value};

use crate::PrintPlugin;

pub struct Register;

impl SimplePluginCommand for Register {
    type Plugin = PrintPlugin;

    fn name(&self) -> &str {
        "tregister"
    }

    fn description(&self) -> &str {
        "Register locales."
    }

    fn signature(&self) -> Signature {
        Signature::build(self.name())
            .required("dir", SyntaxShape::String, "directory containing locales")
            .required("name", SyntaxShape::String, "program name")
    }

    fn run(
        &self,
        _plugin: &Self::Plugin,
        engine: &EngineInterface,
        call: &EvaluatedCall,
        _input: &Value,
    ) -> Result<Value, LabeledError> {
        let path: String = call.req(0).unwrap();
        let mofile: String = call.req(1).unwrap();
        engine
            .add_env_var(
                "NUTEXT_FILES",
                Value::Record {
                    val: Record::from_raw_cols_vals(
                        vec!["path".into(), "name".into()],
                        vec![
                            Value::string(path, Span::unknown()),
                            Value::string(mofile, Span::unknown()),
                        ],
                        Span::unknown(),
                        Span::unknown(),
                    )
                    .unwrap()
                    .into(),
                    internal_span: Span::unknown(),
                },
            )
            .unwrap();
        Ok(Value::nothing(call.head))
    }
}
