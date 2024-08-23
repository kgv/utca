use egui::{Context, Id, Ui};
use fluent::{concurrent::FluentBundle, FluentError, FluentResource};
use fluent_content::Content;
use inflector::Inflector;
use intl_memoizer::concurrent::IntlLangMemoizer;
use std::{
    borrow::Cow,
    fs::read_to_string,
    io,
    ops::Deref,
    sync::{Arc, LazyLock},
};
use thiserror::Error;
use tracing::{enabled, error, Level};
use unic_langid::{langid, LanguageIdentifier, LanguageIdentifierError};

static EN: &[&str] = &[
    include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/ftl/en/fatty_acids.ftl",
    )),
    include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/ftl/en/properties.ftl",
    )),
];

#[derive(Clone)]
pub(crate) struct Localization(pub(crate) Arc<FluentBundle<FluentResource>>);

impl Localization {
    pub(crate) fn get<'a>(&self, key: &'a str) -> Cow<'a, str> {
        self.content(key)
            .map(Cow::Owned)
            .unwrap_or(Cow::Borrowed(key))
    }

    pub(crate) fn get_sentence_case(&self, key: &str) -> String {
        self.get(key).to_sentence_case()
    }
}

impl Deref for Localization {
    type Target = Arc<FluentBundle<FluentResource>>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

/// Extension methods for [`Context`]
pub(crate) trait ContextExt {
    fn localization(&self) -> Localization;
}

impl ContextExt for Context {
    fn localization(&self) -> Localization {
        self.data_mut(|data| {
            data.get_temp_mut_or_insert_with(Id::new("Localization"), || Localization(bundle()))
                .clone()
        })
    }
}

pub(crate) fn bundle() -> Arc<FluentBundle<FluentResource>> {
    println!("localization");
    let bundle = load("en", EN).expect("load localization bundle");
    Arc::new(bundle)
}

pub(crate) fn load(language: &str, sources: &[&str]) -> Result<FluentBundle<FluentResource>> {
    let mut bundle = FluentBundle::new_concurrent(vec![language.parse()?]);
    for &source in sources {
        let resource = match FluentResource::try_new(source.to_owned()) {
            Ok(resource) => resource,
            Err((resource, errors)) => {
                if enabled!(Level::WARN) {
                    for error in errors {
                        error!(%error);
                    }
                }
                resource
            }
        };
        if let Err(errors) = bundle.add_resource(resource) {
            if enabled!(Level::WARN) {
                for error in errors {
                    error!(%error);
                }
            }
        }
    }
    Ok(bundle)
}

/// Result
pub type Result<T, E = Error> = std::result::Result<T, E>;

#[derive(Error, Debug)]
pub enum Error {
    #[error(transparent)]
    LanguageIdentifier(#[from] LanguageIdentifierError),
    #[error(transparent)]
    Io(#[from] io::Error),
}
