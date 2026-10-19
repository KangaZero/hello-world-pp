//! Hello-world snippet for `ProgrammingLanguageName::Yaml`.
//!
//! Imported from leachim6/hello-world (`y/YAML.yaml` at `a152253`),
//! MIT licensed; see `programming_languages/LICENSE-leachim6-hello-world`.

use crate::data::programming_languages::{
    ProgrammingLanguage, ProgrammingLanguageDialect, ProgrammingLanguageName,
    ProgrammingLanguageVersion,
};
use crate::data::{macros::language_names, programming_languages_token::TokenKind};

language_names! {
    language: Yaml => "YAML",
    versions: [Yaml => "YAML"],
    extensions: [],
}

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Yaml,
        extensions: &[],
        categories: &[],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Yaml,
            syntax: r"hello: world",
            tokenized_syntax: &[
                (TokenKind::String, r"hello"),
                (TokenKind::Punctuation, r":"),
                (TokenKind::Plain, r" "),
                (TokenKind::String, r"world"),
            ],
        }],
    };
