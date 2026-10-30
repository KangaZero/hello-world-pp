//! Hello-world snippet for `ProgrammingLanguageName::Charcoal`.
//!
//! Imported from leachim6/hello-world (`c/Charcoal.cl` at `a152253`),
//! MIT licensed; see `programming_languages/LICENSE-leachim6-hello-world`.
//!
//! Sources:
//! - <https://github.com/leachim6/hello-world/blob/a152253b102999bf31988fef077101022e2ad1a5/c/Charcoal.cl>

use crate::data::programming_languages::{
    ProgrammingLanguage, ProgrammingLanguageCategory::Esolang, ProgrammingLanguageDialect,
    ProgrammingLanguageName, ProgrammingLanguageVersion,
};
use crate::data::programming_languages_token::{TokenKind, checked_token_lengths};

use hw_pp_macros::{language_names, output_for_hello_world};

language_names! {
    language: Charcoal => "Charcoal",
    versions: [Charcoal => "Charcoal"],
    extensions: [],
}

const CHARCOAL: &str = output_for_hello_world!();

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Charcoal,
        extensions: &[],
        categories: &[Esolang],
        families: &[],
        paradigms: &[],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Charcoal,
            syntax: CHARCOAL,
            tokenized_syntax: &const {
                checked_token_lengths(CHARCOAL, [(TokenKind::Plain, output_for_hello_world!())])
            },
        }],
    };
