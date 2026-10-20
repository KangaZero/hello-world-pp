//! Hello-world snippet for `ProgrammingLanguageName::NotPython`.
//!
//! Imported from leachim6/hello-world (`n/Not Python` at `a152253`),
//! MIT licensed; see `programming_languages/LICENSE-leachim6-hello-world`.
//!
//! Sources:
//! - <https://github.com/leachim6/hello-world/blob/a152253b102999bf31988fef077101022e2ad1a5/n/Not%20Python>

use crate::data::programming_languages::{
    ProgrammingLanguage, ProgrammingLanguageCategory::Esolang, ProgrammingLanguageDialect,
    ProgrammingLanguageName, ProgrammingLanguageVersion,
};
use crate::data::{
    macros::{language_names, output_for_hello_world},
    programming_languages_token::TokenKind,
};

language_names! {
    language: NotPython => "Not Python",
    versions: [NotPython => "Not Python"],
    extensions: [],
}

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::NotPython,
        extensions: &[],
        categories: &[Esolang],
        families: &[],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::NotPython,
            syntax: concat!(r"{<", output_for_hello_world!(), r"> printer:}"),
            tokenized_syntax: &[(
                TokenKind::Plain,
                concat!(r"{<", output_for_hello_world!(), r"> printer:}"),
            )],
        }],
    };
