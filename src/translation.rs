use fluent_templates::{LanguageIdentifier, langid};

fluent_templates::static_loader! {
    pub(crate) static LOCALES = {
        locales: "./locales",
        fallback_language: "en",
    };
}

pub(crate) const ENGLISH: LanguageIdentifier = langid!("en");
pub(crate) const FRENCH: LanguageIdentifier = langid!("fr");

pub(crate) fn language_from(lang_id: &str) -> LanguageIdentifier {
    match lang_id {
        "en" => ENGLISH,
        "fr" => FRENCH,
        _ => ENGLISH,
    }
}
