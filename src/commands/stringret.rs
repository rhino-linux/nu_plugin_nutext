use std::{
    collections::HashMap,
    fs::{self, File},
    path::PathBuf,
};

use current_locale::current_locale;
use gettext::Catalog;
use locale_match::bcp47::best_matching_locale;
use nu_plugin::{EngineInterface, EvaluatedCall, SimplePluginCommand};
use nu_protocol::{LabeledError, Record, Signature, SyntaxShape, Value};
use strfmt::strfmt;

use crate::PrintPlugin;

pub struct StringRet;

impl SimplePluginCommand for StringRet {
    type Plugin = PrintPlugin;

    fn name(&self) -> &'static str {
        "_"
    }

    fn description(&self) -> &'static str {
        "Return translated string"
    }

    fn search_terms(&self) -> Vec<&str> {
        vec!["gettext", "translation", "i18n", "print", "tregister", "_"]
    }

    fn signature(&self) -> Signature {
        Signature::build(self.name())
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
            env.as_record()?
                .get("path")
                .unwrap()
                .clone()
                .into_string()?,
        );

        let name: String = env
            .as_record()?
            .get("name")
            .unwrap()
            .clone()
            .into_string()?;

        let call_dir: PathBuf = engine.get_current_dir()?.into();

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

        let to_print: String = call.req(0)?;
        let interp_vars: Option<Record> = call.opt(1)?;

        let variable_store = interp_vars
            .unwrap_or_default()
            .into_iter()
            .filter_map(|var| match var.1.coerce_into_string() {
                Ok(o) => Some((var.0, o)),
                Err(_) => None,
            })
            .collect::<HashMap<String, String>>();

        let parsed_vars = match strfmt(catalog.gettext(&to_print), &variable_store) {
            Ok(o) => o,
            Err(e) => {
                return Err(LabeledError::new("Missing variables")
                    .with_help("Did you provide all variables in the string?")
                    .with_inner(LabeledError::new(match e {
                        strfmt::FmtError::Invalid(err)
                        | strfmt::FmtError::KeyError(err)
                        | strfmt::FmtError::TypeError(err) => err,
                    })))
            }
        };

        Ok(Value::String {
            val: parsed_vars,
            internal_span: call.head,
        })
    }
}
