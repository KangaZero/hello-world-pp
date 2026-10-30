//! Hello-world snippet for `ProgrammingLanguageName::Cson`.
//!
//! Imported from leachim6/hello-world (`c/CSON.cson` at `a152253`),
//! MIT licensed; see `programming_languages/LICENSE-leachim6-hello-world`.
//!
//! Sources:
//! - <https://github.com/leachim6/hello-world/blob/a152253b102999bf31988fef077101022e2ad1a5/c/CSON.cson>

use crate::data::programming_languages::{
    FileExtension, ProgrammingLanguage, ProgrammingLanguageCategory::Markup,
    ProgrammingLanguageDialect, ProgrammingLanguageName, ProgrammingLanguageVersion,
};
use crate::data::programming_languages_token::{TokenKind, checked_token_lengths};

use hw_pp_macros::language_names;

language_names! {
    language: Cson => "CSON",
    versions: [Cson => "CSON"],
    extensions: [Cson => "cson"],
}

const CSON: &str = r"{'Hello': 'World'}";

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Cson,
        extensions: &[FileExtension::Cson],
        categories: &[Markup],
        families: &[],
        paradigms: &[],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Cson,
            syntax: CSON,
            tokenized_syntax: &const {
                checked_token_lengths(CSON, [(TokenKind::Plain, r"{'Hello': 'World'}")])
            },
        }],
    };
