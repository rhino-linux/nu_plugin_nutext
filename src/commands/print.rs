use core::fmt;
use std::{error::Error, path::Path, str::FromStr};

use fluent_bundle::FluentArgs;
use fluent_fallback::Localization;
use fluent_resmgr::ResourceManager;
use nu_plugin::{EngineInterface, EvaluatedCall, SimplePluginCommand};
use nu_protocol::{LabeledError, Signature, SyntaxShape, Value};
use unic_langid_impl::LanguageIdentifier;

use crate::FluentPlugin;

struct KeyVal {
    key: String,
    val: String,
}

struct KeyVals(Vec<KeyVal>);

impl FromIterator<KeyVal> for KeyVals {
    fn from_iter<T: IntoIterator<Item = KeyVal>>(iter: T) -> Self {
        Self(iter.into_iter().collect())
    }
}

#[derive(Debug)]
struct KeyValParseErr {
    broken: String,
}

impl Error for KeyValParseErr {}

impl fmt::Display for KeyValParseErr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Could not parse key value set: `{}`", self.broken)
    }
}

impl FromStr for KeyVal {
    type Err = KeyValParseErr;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts: Vec<_> = s.split('=').collect();
        if parts.len() == 2 {
            return Ok(Self {
                key: parts[0].to_string(),
                val: parts[1].to_string(),
            });
        }
        Err(KeyValParseErr { broken: s.into() })
    }
}

impl From<KeyVal> for FluentArgs<'_> {
    fn from(value: KeyVal) -> Self {
        let mut args = Self::new();
        args.set(value.key, value.val);
        args
    }
}

impl From<KeyVals> for FluentArgs<'_> {
    fn from(value: KeyVals) -> Self {
        let mut args = Self::new();
        for i in value.0 {
            args.set(i.key, i.val);
        }
        args
    }
}

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
        engine: &EngineInterface,
        call: &EvaluatedCall,
        _input: &Value,
    ) -> Result<Value, LabeledError> {
        let Ok(Some(env)) = engine.get_env_var("nutext_files") else {
            return Err(LabeledError::new(
                "No env variable `nutext` found! Try `tregister` first.",
            ));
        };

        let path = env
            .as_record()
            .unwrap()
            .get("path")
            .unwrap()
            .clone()
            .into_string()
            .unwrap();

        let files: Vec<String> = env
            .clone()
            .as_record()
            .unwrap()
            .get("files")
            .unwrap()
            .clone()
            .into_list()
            .unwrap()
            .iter()
            .map(|x| x.clone().into_string().unwrap())
            .collect();

        let lang: LanguageIdentifier = match env
            .clone()
            .as_record()
            .unwrap()
            .get("lang")
            .unwrap()
            .clone()
            .into_string()
            .unwrap()
            .parse()
        {
            Ok(o) => o,
            Err(e) => {
                return Err(LabeledError::new(e.to_string())
                    .with_help("Invalid language identifier was passed."));
            }
        };

        let to_print: String = call.req(0).unwrap();
        let interp_vars = call.rest::<String>(1).unwrap_or_default();

        let parsed_vars: Result<KeyVals, LabeledError> = interp_vars
            .iter()
            .map(|x| KeyVal::from_str(x).map_err(|e| LabeledError::new(e.to_string())))
            .collect();

        let parsed_vars = parsed_vars?;

        let abs_path = Path::new(&engine.get_current_dir().unwrap()).join(path);
        let res_mgr = ResourceManager::new(abs_path.to_str().unwrap().to_string());

        let loc = Localization::with_env(
            files.iter().map(|x| x.into()).collect::<Vec<_>>(),
            true,
            vec![lang],
            res_mgr,
        );

        let bundles = loc.bundles();

        let mut errors = vec![];
        let fluent_args: FluentArgs = parsed_vars.into();
        let value = bundles
            .format_value_sync(
                &to_print,
                if fluent_args.iter().count() == 0 {
                    None
                } else {
                    Some(&fluent_args)
                },
                &mut errors,
            )
            .expect("Failed to format the value")
            .unwrap();
        println!("HERE");

        match call.has_flag("stderr") {
            Ok(b) => {
                if b {
                    eprintln!("{value}");
                } else {
                    println!("{value}");
                }
            }
            Err(_) => eprintln!("{value}"),
        }
        Ok(Value::nothing(call.head))
    }
}
