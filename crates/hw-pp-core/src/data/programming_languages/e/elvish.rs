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
    programming_languages_token::{TokenKind, checked_token_lengths},
};

language_names! {
    language: Elvish => "Elvish",
    versions: [Elvish => "Elvish"],
    extensions: [Elv => "elv"],
}

const ELVISH: &str = concat!(
    r#"#!/usr/bin/env elvish
echo ""#,
    output_for_hello_world!(),
    r#"""#
);

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Elvish,
        extensions: &[FileExtension::Elv],
        categories: &[Scripting],
        families: &[],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Elvish,
            syntax: ELVISH,
            tokenized_syntax: &const {
                checked_token_lengths(
                    ELVISH,
                    [
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
                )
            },
        }],
    };
