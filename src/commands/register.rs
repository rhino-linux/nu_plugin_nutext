use nu_plugin::{EngineInterface, EvaluatedCall, SimplePluginCommand};
use nu_protocol::{LabeledError, Record, Signature, Span, SyntaxShape, Value};

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

    fn extra_description(&self) -> &str {
        "`dir` must be in the form: `path/to/{locale}/`, including the literal string `{locale}`"
    }

    fn signature(&self) -> Signature {
        Signature::build(self.name())
            .required("dir", SyntaxShape::String, "directory containing locales")
            .required_named("lang", SyntaxShape::String, "language to use", Some('l'))
            .rest("files", SyntaxShape::String, "translation files in `dir`")
    }

    fn run(
        &self,
        _plugin: &Self::Plugin,
        engine: &EngineInterface,
        call: &EvaluatedCall,
        _input: &Value,
    ) -> Result<Value, LabeledError> {
        let path: String = call.req(0).unwrap();
        let files = call.rest::<String>(1);
        if files.is_err() {
            return Err(LabeledError::new("At least one file must be passed"));
        } else {
            engine
                .add_env_var(
                    "nutext_files",
                    Value::Record {
                        val: Record::from_raw_cols_vals(
                            vec!["path".into(), "files".into(), "lang".into()],
                            vec![
                                Value::string(path, Span::unknown()),
                                Value::list(
                                    files
                                        .unwrap()
                                        .iter()
                                        .map(|x| Value::string(x.to_string(), Span::unknown()))
                                        .collect::<Vec<_>>(),
                                    Span::unknown(),
                                ),
                                Value::string(
                                    call.get_flag::<String>("lang").unwrap().unwrap(),
                                    Span::unknown(),
                                ),
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
        }
        Ok(Value::nothing(call.head))
    }
}
