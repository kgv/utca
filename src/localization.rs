use egui::{RichText, WidgetText};
use fluent::{concurrent::FluentBundle, FluentResource};
use fluent_content::Content;
use std::{
    fmt::{self, Display, Formatter},
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

static RU: &[&str] = &[
    include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/ftl/ru/fatty_acids.ftl",
    )),
    include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/ftl/ru/properties.ftl",
    )),
];

pub(crate) static LOCALIZATION: LazyLock<Localization> = LazyLock::new(|| {
    Localization::new(Locale::En).expect("load en localization bundle")
    // Localization::new(Locale::Ru).expect("load ru localization bundle")
});

macro localized($key:literal) {
    Localized(LazyLock::new(|| localize_or($key)))
}

pub(crate) static ABBREVIATION: Localized = localized!("abbreviation");
pub(crate) static ADDUCT: Localized = localized!("adduct");
pub(crate) static ASCENDING_DESCRIPTION: Localized = localized!("ascending_description");
pub(crate) static ASCENDING: Localized = localized!("ascending");
pub(crate) static CALCULATION: Localized = localized!("calculation");
pub(crate) static COMMON_NAME: Localized = localized!("common_name");
pub(crate) static COMPOSITION: Localized = localized!("composition");
pub(crate) static CONFIGURATION: Localized = localized!("configuration");
pub(crate) static DAG: Localized = localized!("dag");
pub(crate) static DESCENDING_DESCRIPTION: Localized = localized!("descending_description");
pub(crate) static DESCENDING: Localized = localized!("descending");
pub(crate) static DIACYLGLYCEROL: Localized = localized!("diacylglycerol");
pub(crate) static EDIT: Localized = localized!("edit");
pub(crate) static EXPERIMENTAL: Localized = localized!("experimental");
pub(crate) static FA: Localized = localized!("fa");
pub(crate) static FATTY_ACID_MASS: Localized = localized!("fatty_acid_mass");
pub(crate) static FATTY_ACID: Localized = localized!("fatty_acid");
pub(crate) static FORMULA: Localized = localized!("formula");
pub(crate) static FRACTION: Localized = localized!("fraction");
pub(crate) static GUNSTONE_DESCRIPTION: Localized = localized!("gunstone_description");
pub(crate) static GUNSTONE: Localized = localized!("gunstone");
pub(crate) static KEY_DESCRIPTION: Localized = localized!("key_description");
pub(crate) static KEY: Localized = localized!("key");
pub(crate) static LEFT_PANEL: Localized = localized!("left_panel");
pub(crate) static MAG: Localized = localized!("mag");
pub(crate) static MASS_FRACTION: Localized = localized!("mass_fraction");
pub(crate) static METHOD: Localized = localized!("method");
pub(crate) static METHYL_ESTER_MASS: Localized = localized!("methyl_ester_mass");
pub(crate) static MIXTURE_MOLAR_MASS: Localized = localized!("mixture_molar_mass");
pub(crate) static MOLE_FRACTION: Localized = localized!("mole_fraction");
pub(crate) static MONOACYLGLYCEROL: Localized = localized!("monoacylglycerol");
pub(crate) static NAMES_DESCRIPTION: Localized = localized!("names_description");
pub(crate) static NAMES: Localized = localized!("names");
pub(crate) static ORDER: Localized = localized!("order");
pub(crate) static PERCENT: Localized = localized!("percent");
pub(crate) static PRECISION: Localized = localized!("precision");
pub(crate) static PROPERTIES_DESCRIPTION: Localized = localized!("properties_description");
pub(crate) static PROPERTIES: Localized = localized!("properties");
pub(crate) static RESET_APPLICATION: Localized = localized!("reset_application");
pub(crate) static RESET_GUI: Localized = localized!("reset_gui");
pub(crate) static RESIZE: Localized = localized!("resize");
pub(crate) static SELECTIVITY_FACTOR: Localized = localized!("selectivity_factor");
pub(crate) static SIGN: Localized = localized!("sign");
pub(crate) static SIGNED_DESCRIPTION: Localized = localized!("signed_description");
pub(crate) static SIGNED: Localized = localized!("signed");
pub(crate) static SORT: Localized = localized!("sort");
pub(crate) static SYSTEMATIC_NAME: Localized = localized!("systematic_name");
pub(crate) static TAG: Localized = localized!("tag");
pub(crate) static THEORETICAL: Localized = localized!("theoretical");
pub(crate) static TRIACYLGLYCEROL: Localized = localized!("triacylglycerol");
pub(crate) static UNSIGNED_DESCRIPTION: Localized = localized!("unsigned_description");
pub(crate) static UNSIGNED: Localized = localized!("unsigned");
pub(crate) static VALUE_DESCRIPTION: Localized = localized!("value_description");
pub(crate) static VALUE: Localized = localized!("value");
pub(crate) static VANDER_WAL_DESCRIPTION: Localized = localized!("vander_wal_description");
pub(crate) static VANDER_WAL: Localized = localized!("vander_wal");

// pub(crate) static MASS: Localized = localized!("mass");
// pub(crate) static MOLAR_MASS: Localized = localized!("molar_mass");

pub(crate) fn localize(key: &str) -> Option<String> {
    LOCALIZATION.0.content(key)
}

// pub(crate) fn localize_or(key: &str) -> String {
//     let text = LOCALIZATION
//         .0
//         .content(key)
//         .map(Cow::Owned)
//         .unwrap_or(Cow::Borrowed(key));
//     let mut chars = text.chars();
//     chars
//         .next()
//         .map(char::to_uppercase)
//         .into_iter()
//         .flatten()
//         .chain(chars)
//         .collect()
// }
pub(crate) fn localize_or(key: &str) -> String {
    match LOCALIZATION.0.content(key) {
        Some(content) => {
            let mut chars = content.chars();
            chars
                .next()
                .map(char::to_uppercase)
                .into_iter()
                .flatten()
                .chain(chars)
                .collect()
        }
        None => key.to_uppercase(),
    }
}

/// Localization
#[derive(Clone)]
pub(crate) struct Localization(Arc<FluentBundle<FluentResource>>);

impl Localization {
    fn new(locale: Locale) -> Result<Self> {
        let mut bundle = FluentBundle::new_concurrent(vec![locale.into()]);
        let sources = match locale {
            Locale::En => EN,
            Locale::Ru => RU,
        };
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
        Ok(Localization(Arc::new(bundle)))
    }
}

/// Localized
pub(crate) struct Localized(LazyLock<String>);

impl Deref for Localized {
    type Target = String;

    fn deref(&self) -> &Self::Target {
        &*self.0
    }
}

impl Display for Localized {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        Display::fmt(&*self.0, f)
    }
}

impl<'a> From<&'a Localized> for &'a str {
    fn from(value: &'a Localized) -> Self {
        &*value.0
    }
}

impl From<&Localized> for String {
    fn from(value: &Localized) -> Self {
        (&*value.0).into()
    }
}

impl From<&Localized> for RichText {
    fn from(value: &Localized) -> Self {
        RichText::new(value)
    }
}

impl From<&Localized> for WidgetText {
    fn from(value: &Localized) -> Self {
        WidgetText::RichText(value.into())
    }
}

/// Locale
#[derive(Clone, Copy, Debug, Default)]
enum Locale {
    Ru,
    #[default]
    En,
}

impl From<Locale> for LanguageIdentifier {
    fn from(value: Locale) -> Self {
        match value {
            Locale::Ru => langid!("ru"),
            Locale::En => langid!("en"),
        }
    }
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
