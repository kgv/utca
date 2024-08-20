use fluent::{concurrent::FluentBundle, FluentError, FluentResource};
use intl_memoizer::concurrent::IntlLangMemoizer;
use std::{fs::read_to_string, io, sync::Arc};
use thiserror::Error;
use tracing::{enabled, error, Level};
use unic_langid::{langid, LanguageIdentifier, LanguageIdentifierError};

pub(crate) type Localization = Arc<FluentBundle<FluentResource>>;

pub(crate) fn bundle() -> Result<Arc<FluentBundle<FluentResource>>> {
    println!("localization");
    let bundle = load("en", ["fatty_acids.ftl", "properties.ftl"])?;
    Ok(Arc::new(bundle))
}

pub(crate) fn load(
    language: &str,
    files: impl IntoIterator<Item = &str>,
) -> Result<FluentBundle<FluentResource>> {
    let mut bundle = FluentBundle::new_concurrent(vec![language.parse()?]);
    for file in files {
        let path = format!("ftl/{language}/{file}");
        let source = read_to_string(path)?;
        let resource = match FluentResource::try_new(source) {
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
