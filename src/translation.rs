use convert_case::{Case, Casing};
use fluent_templates::{LanguageIdentifier, Loader, langid};
use std::fmt::Debug;

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

pub trait TypeName: Debug {
    fn snake_case_type(&self) -> String {
        self.type_name().to_case(Case::Snake)
    }

    fn type_name(&self) -> String {
        format!("{:?}", self)
    }
}

pub trait TranslatedName: TypeName {
    fn lower_name(&self, lang_id: &str) -> String {
        self.name(lang_id).to_case(Case::Lower)
    }

    fn name(&self, lang_id: &str) -> String {
        LOCALES.lookup(&language_from(lang_id), &*self.type_name())
    }

    fn description(&self, lang_id: &str) -> String {
        LOCALES.lookup(
            &language_from(lang_id),
            &format!("{}_description", &*self.type_name()),
        )
    }

    fn first_letter(&self, lang_id: &str) -> String {
        let mut result = String::new();

        for word in self.name(lang_id).split(" ") {
            let first_letter = word.chars().next().unwrap();
            result.push(first_letter);
        }

        result
    }
}
