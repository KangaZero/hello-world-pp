//! Hello-world snippet for `ProgrammingLanguageName::Hello`.
//!
//! Imported from leachim6/hello-world (`h/Hello` at `a152253`),
//! MIT licensed; see `programming_languages/LICENSE-leachim6-hello-world`.
//!
//! Sources:
//! - <https://github.com/leachim6/hello-world/blob/a152253b102999bf31988fef077101022e2ad1a5/h/Hello>

use crate::data::programming_languages::{
    ProgrammingLanguage, ProgrammingLanguageCategory::Esolang, ProgrammingLanguageDialect,
    ProgrammingLanguageName, ProgrammingLanguageVersion,
};
use crate::data::programming_languages_token::{TokenKind, checked_token_lengths};

use hw_pp_macros::language_names;

language_names! {
    language: Hello => "Hello",
    versions: [Hello => "Hello"],
    extensions: [],
}

const HELLO: &str = r"h";

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Hello,
        extensions: &[],
        categories: &[Esolang],
        families: &[],
        paradigms: &[],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Hello,
            syntax: HELLO,
            tokenized_syntax: &const { checked_token_lengths(HELLO, [(TokenKind::Plain, r"h")]) },
        }],
    };
