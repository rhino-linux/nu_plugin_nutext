use std::{
    collections::HashMap,
    fs::{self, File},
    path::PathBuf,
};

use current_locale::current_locale;
use gettext::Catalog;
use locale_match::bcp47::best_matching_locale;
use nu_plugin::{EngineInterface, EvaluatedCall, SimplePluginCommand};
use nu_protocol::{Example, LabeledError, Record, Signature, SyntaxShape, Value};
use strfmt::strfmt;

use crate::PrintPlugin;

pub struct Print;

impl SimplePluginCommand for Print {
    type Plugin = PrintPlugin;

    fn name(&self) -> &str {
        "tprint"
    }

    fn description(&self) -> &str {
        "Print out translated strings."
    }

    fn examples(&self) -> Vec<nu_protocol::Example> {
        vec![Example {
            description: "Print a translated string",
            example: r#"tprint "Hello {name}" { name: "Foo" }"#,
            result: None,
        }]
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
        let Ok(Some(env)) = engine.get_env_var("NUTEXT_FILES") else {
            return Err(LabeledError::new("No env variable `NUTEXT_FILES` found!")
                .with_help("Try `tregister`.")
                .with_label("Here", call.head));
        };

        let path: PathBuf = PathBuf::from(
            env.clone()
                .as_record()
                .unwrap()
                .get("path")
                .unwrap()
                .clone()
                .into_string()
                .unwrap(),
        );

        let name: String = env
            .clone()
            .as_record()
            .unwrap()
            .get("name")
            .unwrap()
            .clone()
            .into_string()
            .unwrap();

        let call_dir: PathBuf = engine.get_current_dir().unwrap().into();

        let available_locales: Vec<String> = fs::read_dir(call_dir.join(&path))
            .expect("Could not open path")
            .filter_map(|entry| {
                entry.ok().and_then(|e| {
                    let path = e.path();
                    if path.is_dir() {
                        path.file_name()
                            .and_then(|name| name.to_str().map(String::from))
                    } else {
                        None
                    }
                })
            })
            .collect();

        let catalog = match File::open(
            call_dir
                .join(path)
                .join(
                    best_matching_locale(available_locales, current_locale())
                        .unwrap_or("en-US".into()),
                )
                .join("LC_MESSAGES")
                .join(format!("{name}.mo")),
        ) {
            // If there are *any* errors, just let's just default back.
            Ok(o) => Catalog::parse(o).unwrap_or(Catalog::empty()),
            Err(_) => Catalog::empty(),
        };

        let to_print: String = call
            .req(0)
            .expect("Why didn't nu catch this in the signature?");
        let interp_vars: Option<Record> = call
            .opt(1)
            .expect("Why didn't nu catch this in the signature?");

        let variable_store: HashMap<String, String> = match interp_vars {
            Some(vars) => vars
                .iter()
                .filter_map(|var| match var.1.coerce_string() {
                    Ok(o) => Some((var.0.to_owned(), o)),
                    Err(_) => None,
                })
                .collect(),
            None => HashMap::new(),
        };

        let parsed_vars = match strfmt(catalog.gettext(&to_print), &variable_store) {
            Ok(o) => o,
            Err(e) => {
                return Err(LabeledError::new("Missing variables")
                    .with_help("Did you provide all variables in the string?")
                    .with_inner(LabeledError::new(e.to_string()).with_label("Here", call.head)))
            }
        };

        if call.has_flag("stderr").unwrap_or(false) {
            if call.has_flag("no-newline").unwrap_or(false) {
                eprint!("{parsed_vars}");
            } else {
                eprintln!("{parsed_vars}");
            };
        } else if call.has_flag("no-newline").unwrap_or(false) {
            print!("{parsed_vars}");
        } else {
            println!("{parsed_vars}");
        }

        Ok(Value::nothing(call.head))
    }
}
