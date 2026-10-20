//! Hello-world snippet for `ProgrammingLanguageName::Elvish`.
//!
//! Imported from leachim6/hello-world (`e/Elvish.elv` at `a152253`),
//! MIT licensed; see `programming_languages/LICENSE-leachim6-hello-world`.
//!
//! Sources:
//! - <https://github.com/leachim6/hello-world/blob/a152253b102999bf31988fef077101022e2ad1a5/e/Elvish.elv>

use crate::data::programming_languages::{
    FileExtension, ProgrammingLanguage, ProgrammingLanguageCategory::Scripting,
    ProgrammingLanguageDialect, ProgrammingLanguageName, ProgrammingLanguageVersion,
};
use crate::data::{
    macros::{language_names, output_for_hello_world},
    programming_languages_token::TokenKind,
};

language_names! {
    language: Elvish => "Elvish",
    versions: [Elvish => "Elvish"],
    extensions: [Elv => "elv"],
}

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Elvish,
        extensions: &[FileExtension::Elv],
        categories: &[Scripting],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Elvish,
            syntax: concat!(
                r#"#!/usr/bin/env elvish
echo ""#,
                output_for_hello_world!(),
                r#"""#
            ),
            tokenized_syntax: &[
                (TokenKind::Comment, r"#!/usr/bin/env elvish"),
                (
                    TokenKind::Plain,
                    r"
",
                ),
                (TokenKind::Keyword, r"echo"),
                (TokenKind::Plain, r" "),
                (
                    TokenKind::String,
                    concat!(r#"""#, output_for_hello_world!(), r#"""#),
                ),
            ],
        }],
    };
