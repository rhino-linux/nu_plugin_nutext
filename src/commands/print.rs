use nu_plugin::{EngineInterface, EvaluatedCall, SimplePluginCommand};
use nu_protocol::{Example, LabeledError, PipelineData, Signature, SyntaxShape, Value};

use crate::PrintPlugin;

pub struct Print;

impl SimplePluginCommand for Print {
    type Plugin = PrintPlugin;

    fn name(&self) -> &'static str {
        "tprint"
    }

    fn description(&self) -> &'static str {
        "Print out translated strings."
    }

    fn examples(&self) -> Vec<nu_protocol::Example> {
        vec![Example {
            description: "Print a translated string",
            example: r#"tprint "Hello {name}" { name: "Foo" }"#,
            result: None,
        }]
    }

    fn search_terms(&self) -> Vec<&str> {
        vec!["gettext", "translation", "i18n", "print", "tregister", "_"]
    }

    fn signature(&self) -> Signature {
        Signature::build(self.name())
            .switch("stderr", "print to stderr instead of stdout", Some('e'))
            .switch(
                "no-newline",
                "print without inserting a newline for the line ending",
                Some('n'),
            )
            .required("key", SyntaxShape::String, "gettext key")
            .optional(
                "vars",
                SyntaxShape::Record(vec![]),
                "interpolation variables",
            )
    }

    fn run(
        &self,
        _plugin: &Self::Plugin,
        engine: &EngineInterface,
        call: &EvaluatedCall,
        _input: &Value,
    ) -> Result<Value, LabeledError> {
        if let Some(decl_id) = engine.find_decl("_")? {
            // Pass our data into `_` and get back the data to print later.
            match engine.call_decl(decl_id, call.clone(), PipelineData::empty(), true, true)? {
                PipelineData::Value(val, _) => {
                    let val = val
                        .coerce_string()
                        .expect("Could not coerce output into string");

                    match (
                        call.has_flag("stderr").unwrap_or(false),
                        call.has_flag("no-newline").unwrap_or(false),
                    ) {
                        (true, true) => eprint!("{val}"),
                        (true, false) => eprintln!("{val}"),
                        (false, true) => print!("{val}"),
                        (false, false) => println!("{val}"),
                    }
                    Ok(Value::nothing(call.head))
                }
                _ => {
                    unimplemented!("You may be using mismatched features between nutext versions!")
                }
            }
        } else {
            Err(LabeledError::new("Could not find `_`"))
        }
    }
}
