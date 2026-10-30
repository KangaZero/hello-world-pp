//! Hello-world snippet for `ProgrammingLanguageName::Yaml`.
//!
//! Imported from leachim6/hello-world (`y/YAML.yaml` at `a152253`),
//! MIT licensed; see `programming_languages/LICENSE-leachim6-hello-world`.
//!
//! Sources:
//! - <https://github.com/leachim6/hello-world/blob/a152253b102999bf31988fef077101022e2ad1a5/y/YAML.yaml>

use crate::data::programming_languages::{
    ProgrammingLanguage, ProgrammingLanguageCategory::Markup, ProgrammingLanguageDialect,
    ProgrammingLanguageName, ProgrammingLanguageVersion,
};
use crate::data::programming_languages_token::{TokenKind, checked_token_lengths};

use hw_pp_macros::language_names;

language_names! {
    language: Yaml => "YAML",
    versions: [Yaml => "YAML"],
    extensions: [],
}

const YAML: &str = r"hello: world";

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Yaml,
        extensions: &[],
        categories: &[Markup],
        families: &[],
        paradigms: &[],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Yaml,
            syntax: YAML,
            tokenized_syntax: &const {
                checked_token_lengths(
                    YAML,
                    [
                        (TokenKind::String, r"hello"),
                        (TokenKind::Punctuation, r":"),
                        (TokenKind::Plain, r" "),
                        (TokenKind::String, r"world"),
                    ],
                )
            },
        }],
    };
