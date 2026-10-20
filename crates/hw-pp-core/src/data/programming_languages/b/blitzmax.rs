//! Hello-world snippet for `ProgrammingLanguageName::Blitzmax`.
//!
//! Imported from leachim6/hello-world (PR #1524 `b/BlitzMax.bmx`),
//! MIT licensed; see `programming_languages/LICENSE-leachim6-hello-world`.
//!
//! Sources:
//! - <https://github.com/leachim6/hello-world/pull/1524>

use crate::data::programming_languages::{
    FileExtension, ProgrammingLanguage, ProgrammingLanguageCategory::Compiled,
    ProgrammingLanguageDialect, ProgrammingLanguageName, ProgrammingLanguageVersion,
};
use crate::data::{
    macros::{language_names, output_for_hello_world},
    programming_languages_token::TokenKind,
};

language_names! {
    language: Blitzmax => "BlitzMax",
    versions: [Blitzmax => "BlitzMax"],
    extensions: [Bmx => "bmx"],
}

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Blitzmax,
        extensions: &[FileExtension::Bmx],
        categories: &[Compiled],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Blitzmax,
            syntax: concat!(r#"Print ""#, output_for_hello_world!(), r#"""#),
            tokenized_syntax: &[
                (TokenKind::Keyword, r"Print"),
                (TokenKind::Plain, r" "),
                (
                    TokenKind::String,
                    concat!(r#"""#, output_for_hello_world!(), r#"""#),
                ),
            ],
        }],
    };
